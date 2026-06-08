"use client";

import { useState } from "react";
import { encodeFunctionData, type Address, type Hex } from "viem";
import { useAccount, useChainId, useSignTypedData, useSwitchChain, useWriteContract } from "wagmi";
import { cctpAdapterAbi, oftAdapterAbi } from "@/lib/abis/generated";
import { erc20Abi } from "@/lib/abis/erc20";
import { erc20PermitAbi } from "@/lib/abis/erc20Permit";
import { REGISTRY, type SupportedChainId } from "@/lib/chains";
import { adapterAddress } from "@/lib/config";
import { fmtAmount } from "@/lib/format";
import { bytes32ToAddress, BRIDGE_CCTP, type Order } from "@/lib/order";
import { buildPermit2612TypedData, splitSignature } from "@/lib/permit2612";
import { readTokenDomain } from "@/lib/tokenPermit";
import { publicClientFor } from "@/lib/viemClients";

export type SelfFillPhase =
  | "idle"
  | "checking"
  | "switching"
  | "signing"
  | "filling"
  | "confirming"
  | "done"
  | "error";

export interface SelfFillState {
  phase: SelfFillPhase;
  message?: string;
  error?: string;
}

const secsFromNow = (s: number) => BigInt(Math.floor(Date.now() / 1000) + s);

/**
 * Optimistically fill the user's OWN order on the destination chain — the same action a relayer
 * performs, exposed for self-service. Mirrors the EIP-2612 initiate flow: a single
 * `multicall([selfPermit(outputToken), fill(order)])` so the filler approves and fills in one tx.
 * The filler fronts the full `outputAmount` and is reimbursed when the underlying bridge settles.
 */
export function useSelfFill() {
  const { address } = useAccount();
  const connectedChainId = useChainId();
  const { switchChainAsync } = useSwitchChain();
  const { writeContractAsync } = useWriteContract();
  const { signTypedDataAsync } = useSignTypedData();
  const [state, setState] = useState<SelfFillState>({ phase: "idle" });

  const reset = () => setState({ phase: "idle" });

  async function selfFill(order: Order): Promise<Hex> {
    const dst = order.dstChainId as SupportedChainId;
    const bridgeType = order.bridgeType;
    const adapter = adapterAddress(bridgeType);
    const abi = bridgeType === BRIDGE_CCTP ? cctpAdapterAbi : oftAdapterAbi;
    const token = bytes32ToAddress(order.outputToken);
    const filler = address as Address | undefined;
    const decimals = REGISTRY[dst].usdc.decimals; // both supported tokens are 6dp

    try {
      if (!filler) throw new Error("Connect a wallet to self-relay");

      // Liquidity: the filler fronts the worst-case payout (outputAmount) on the destination.
      setState({ phase: "checking", message: "Checking destination balance" });
      const client = publicClientFor(dst);
      const balance = (await client.readContract({
        address: token,
        abi: erc20Abi,
        functionName: "balanceOf",
        args: [filler],
      })) as bigint;
      if (balance < order.outputAmount) {
        throw new Error(
          `Need ${fmtAmount(order.outputAmount, decimals, 4)} on ${REGISTRY[dst].shortName} to fill (have ${fmtAmount(balance, decimals, 4)})`,
        );
      }

      if (connectedChainId !== dst) {
        setState({ phase: "switching", message: `Switch to ${REGISTRY[dst].shortName}` });
        await switchChainAsync({ chainId: dst });
      }

      // EIP-2612 permit binding the output token to the adapter, batched with the fill.
      setState({ phase: "signing", message: "Sign the token permit" });
      const { name, version } = await readTokenDomain(token, dst);
      const nonce = (await client.readContract({
        address: token,
        abi: erc20PermitAbi,
        functionName: "nonces",
        args: [filler],
      })) as bigint;
      const deadline = secsFromNow(3600);
      const signature = await signTypedDataAsync(
        buildPermit2612TypedData({
          name,
          version,
          chainId: dst,
          token,
          owner: filler,
          spender: adapter,
          value: order.outputAmount,
          nonce,
          deadline,
        }),
      );
      const { v, r, s } = splitSignature(signature);
      const selfPermitData = encodeFunctionData({ abi, functionName: "selfPermit", args: [token, order.outputAmount, deadline, v, r, s] });
      const fillData = encodeFunctionData({ abi, functionName: "fill", args: [order] });

      setState({ phase: "filling", message: "Confirm the fill in your wallet" });
      const hash = await writeContractAsync({
        address: adapter,
        abi,
        functionName: "multicall",
        args: [[selfPermitData, fillData]],
        chainId: dst,
      });

      setState({ phase: "confirming", message: "Waiting for confirmation" });
      await client.waitForTransactionReceipt({ hash });
      setState({ phase: "done", message: "Filled" });
      return hash;
    } catch (e) {
      const msg = e instanceof Error ? e.message : String(e);
      const friendly = /OrderAlreadyActive/i.test(msg)
        ? "This order was already filled or settled."
        : msg;
      setState({ phase: "error", error: friendly.slice(0, 200) });
      throw e;
    }
  }

  return { selfFill, state, reset };
}
