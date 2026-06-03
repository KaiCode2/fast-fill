"use client";

import { useState } from "react";
import { encodeFunctionData, keccak256, maxUint256, parseEventLogs, type Address, type Hex } from "viem";
import { useAccount, useChainId, useSignTypedData, useSwitchChain, useWriteContract } from "wagmi";
import { cctpAdapterAbi, oftAdapterAbi } from "@/lib/abis/generated";
import { erc20Abi } from "@/lib/abis/erc20";
import { erc20PermitAbi } from "@/lib/abis/erc20Permit";
import { oftAbi } from "@/lib/abis/oft";
import { api, type SubmitMode } from "@/lib/api";
import { buildOrder, cctpInitiateArgs, oftInitiateArgs, outputAmountOf, type BridgeParams } from "@/lib/bridge";
import { PERMIT2, REGISTRY, type SupportedChainId } from "@/lib/chains";
import { adapterAddress } from "@/lib/config";
import { buildExtraOptions, DEFAULT_COMPOSE_GAS, DEFAULT_LZRECEIVE_GAS } from "@/lib/lzOptions";
import {
  addressToBytes32,
  BRIDGE_CCTP,
  encodeOrder,
  orderIdOf,
  serializeOrder,
  type Order,
} from "@/lib/order";
import { buildOrderIntentTypedData, cctpBridgeParams, oftBridgeParams, randomPermit2Nonce } from "@/lib/permit2";
import { buildPermit2612TypedData, splitSignature } from "@/lib/permit2612";
import { getToken } from "@/lib/tokens";
import { publicClientFor } from "@/lib/viemClients";

export type InitiatePhase =
  | "idle"
  | "switching"
  | "approving"
  | "quoting"
  | "signing"
  | "submitting"
  | "confirming"
  | "handoff"
  | "done"
  | "error";

export interface InitiateState {
  phase: InitiatePhase;
  message?: string;
  error?: string;
}

export interface InitiateResult {
  orderId: Hex;
  srcTxHash: Hex;
  order?: Order;
}

const secsFromNow = (s: number) => BigInt(Math.floor(Date.now() / 1000) + s);

export function useInitiate() {
  const { address } = useAccount();
  const connectedChainId = useChainId();
  const { switchChainAsync } = useSwitchChain();
  const { writeContractAsync } = useWriteContract();
  const { signTypedDataAsync } = useSignTypedData();
  const [state, setState] = useState<InitiateState>({ phase: "idle" });

  const reset = () => setState({ phase: "idle" });

  async function ensureChain(chainId: SupportedChainId) {
    if (connectedChainId !== chainId) {
      setState({ phase: "switching", message: `Switch to ${REGISTRY[chainId].shortName}` });
      await switchChainAsync({ chainId });
    }
  }

  async function quoteOftFee(p: BridgeParams, sender: Address): Promise<bigint> {
    const client = publicClientFor(p.srcChainId);
    const oft = REGISTRY[p.srcChainId].usdt0Oft!;
    const adapter = adapterAddress(p.bridgeType);
    const preview = buildOrder(p, sender, { startTime: secsFromNow(0) });
    const sendParam = {
      dstEid: REGISTRY[p.dstChainId].lzEid,
      to: addressToBytes32(adapter),
      amountLD: p.amount,
      minAmountLD: outputAmountOf(p),
      extraOptions: buildExtraOptions(),
      composeMsg: encodeOrder(preview),
      oftCmd: "0x" as Hex,
    };
    const fee = (await client.readContract({
      address: oft,
      abi: oftAbi,
      functionName: "quoteSend",
      args: [sendParam, false],
    })) as { nativeFee: bigint };
    return (fee.nativeFee * 12n) / 10n;
  }

  async function readTokenDomain(token: Address, chainId: SupportedChainId): Promise<{ name: string; version: string }> {
    const client = publicClientFor(chainId);
    try {
      // EIP-5267 returns (fields, name, version, chainId, verifyingContract, salt, extensions).
      const d = (await client.readContract({
        address: token,
        abi: erc20PermitAbi,
        functionName: "eip712Domain",
      })) as readonly [Hex, string, string, bigint, Address, Hex, readonly bigint[]];
      return { name: d[1], version: d[2] };
    } catch {
      /* token doesn't implement EIP-5267 */
    }
    const name = (await client.readContract({ address: token, abi: erc20Abi, functionName: "name" })) as string;
    let version = "1";
    try {
      version = (await client.readContract({ address: token, abi: erc20PermitAbi, functionName: "version" })) as string;
    } catch {
      /* default to "1" */
    }
    return { name, version };
  }

  /** Shared tail for self-submitted txs: confirm, parse OrderCreated, reconstruct, hand off. */
  async function finishSelfSubmitted(p: BridgeParams, txHash: Hex, forwarding: boolean): Promise<InitiateResult> {
    const client = publicClientFor(p.srcChainId);
    const adapter = adapterAddress(p.bridgeType);
    setState({ phase: "confirming", message: "Waiting for confirmation" });
    const receipt = await client.waitForTransactionReceipt({ hash: txHash });
    const abi = p.bridgeType === BRIDGE_CCTP ? cctpAdapterAbi : oftAdapterAbi;
    const events = parseEventLogs({ abi, eventName: "OrderCreated", logs: receipt.logs });
    const created = events.find((e) => (e.address as string).toLowerCase() === adapter.toLowerCase());
    if (!created) throw new Error("OrderCreated event not found in the transfer receipt");
    const emittedOrderId = (created.args as { orderId: Hex }).orderId;
    const nonce = (created.args as { nonce: bigint }).nonce;
    const block = await client.getBlock({ blockNumber: receipt.blockNumber });

    const order = buildOrder(p, address as Address, { nonce, startTime: block.timestamp });
    const orderId = orderIdOf(order);
    if (orderId.toLowerCase() !== emittedOrderId.toLowerCase()) {
      throw new Error("Reconstructed orderId does not match the on-chain event.");
    }

    setState({ phase: "handoff", message: "Notifying the relayer" });
    try {
      await api.submitSelf({ txHash, srcChainId: p.srcChainId, forwarding, order: serializeOrder(order) });
    } catch (e) {
      console.warn("relayer handoff failed:", e);
    }
    setState({ phase: "done", message: "Submitted" });
    return { orderId, order, srcTxHash: txHash };
  }

  async function ensureAllowance(token: Address, owner: Address, spender: Address, amount: bigint, chainId: SupportedChainId) {
    const client = publicClientFor(chainId);
    const allowance = (await client.readContract({
      address: token,
      abi: erc20Abi,
      functionName: "allowance",
      args: [owner, spender],
    })) as bigint;
    if (allowance >= amount) return;
    const hash = await writeContractAsync({
      address: token,
      abi: erc20Abi,
      functionName: "approve",
      args: [spender, maxUint256],
      chainId,
    });
    await client.waitForTransactionReceipt({ hash });
  }

  // -- Mode (a): standard approve + initiate -----------------------------------------------------
  async function runStandard(p: BridgeParams, forwarding: boolean): Promise<InitiateResult> {
    const sender = address as Address;
    const adapter = adapterAddress(p.bridgeType);
    const inputToken = getToken(p.srcChainId, p.token).address;
    await ensureChain(p.srcChainId);

    setState({ phase: "approving", message: `Checking ${p.token} allowance` });
    await ensureAllowance(inputToken, sender, adapter, p.amount, p.srcChainId);

    let value = 0n;
    if (p.bridgeType !== BRIDGE_CCTP) {
      setState({ phase: "quoting", message: "Quoting LayerZero fee" });
      value = await quoteOftFee(p, sender);
    }

    setState({ phase: "submitting", message: "Confirm the transfer in your wallet" });
    const txHash =
      p.bridgeType === BRIDGE_CCTP
        ? await writeContractAsync({ address: adapter, abi: cctpAdapterAbi, functionName: "initiateCCTP", args: cctpInitiateArgs(p), chainId: p.srcChainId })
        : await writeContractAsync({ address: adapter, abi: oftAdapterAbi, functionName: "initiateOFT", args: oftInitiateArgs(p, buildExtraOptions()), chainId: p.srcChainId, value });

    return finishSelfSubmitted(p, txHash, forwarding);
  }

  // -- Mode (b): EIP-2612 single tx: multicall([selfPermit, initiate]) ----------------------------
  async function runPermit2612(p: BridgeParams, forwarding: boolean): Promise<InitiateResult> {
    const sender = address as Address;
    const adapter = adapterAddress(p.bridgeType);
    const token = getToken(p.srcChainId, p.token).address;
    const abi = p.bridgeType === BRIDGE_CCTP ? cctpAdapterAbi : oftAdapterAbi;
    await ensureChain(p.srcChainId);

    setState({ phase: "signing", message: `Sign the ${p.token} permit` });
    const { name, version } = await readTokenDomain(token, p.srcChainId);
    const permitNonce = (await publicClientFor(p.srcChainId).readContract({
      address: token,
      abi: erc20PermitAbi,
      functionName: "nonces",
      args: [sender],
    })) as bigint;
    const deadline = secsFromNow(3600);
    const signature = await signTypedDataAsync(
      buildPermit2612TypedData({ name, version, chainId: p.srcChainId, token, owner: sender, spender: adapter, value: p.amount, nonce: permitNonce, deadline }),
    );
    const { v, r, s } = splitSignature(signature);

    const selfPermitData = encodeFunctionData({ abi, functionName: "selfPermit", args: [token, p.amount, deadline, v, r, s] });
    let value = 0n;
    const initiateData =
      p.bridgeType === BRIDGE_CCTP
        ? encodeFunctionData({ abi: cctpAdapterAbi, functionName: "initiateCCTP", args: cctpInitiateArgs(p) })
        : (() => encodeFunctionData({ abi: oftAdapterAbi, functionName: "initiateOFT", args: oftInitiateArgs(p, buildExtraOptions()) }))();
    if (p.bridgeType !== BRIDGE_CCTP) {
      setState({ phase: "quoting", message: "Quoting LayerZero fee" });
      value = await quoteOftFee(p, sender);
    }

    setState({ phase: "submitting", message: "Confirm in your wallet (one transaction)" });
    const txHash = await writeContractAsync({
      address: adapter,
      abi,
      functionName: "multicall",
      args: [[selfPermitData, initiateData]],
      chainId: p.srcChainId,
      value,
    });
    return finishSelfSubmitted(p, txHash, forwarding);
  }

  // -- Mode (c): Permit2 gasless sponsored intent -------------------------------------------------
  async function runPermit2(p: BridgeParams, forwarding: boolean): Promise<InitiateResult> {
    const from = address as Address;
    const adapter = adapterAddress(p.bridgeType);
    const token = getToken(p.srcChainId, p.token).address;
    await ensureChain(p.srcChainId);

    setState({ phase: "approving", message: "One-time Permit2 approval (if needed)" });
    await ensureAllowance(token, from, PERMIT2, p.amount, p.srcChainId);

    const outputAmount = outputAmountOf(p);
    const extraOptions = buildExtraOptions();
    const bridgeParams =
      p.bridgeType === BRIDGE_CCTP ? cctpBridgeParams(p.maxFee, p.minFinalityThreshold) : oftBridgeParams(extraOptions);
    const hookDataHash = keccak256(p.hookData);
    const nonce = randomPermit2Nonce();
    const deadline = secsFromNow(3600);
    const recipient = addressToBytes32(p.recipient);

    setState({ phase: "signing", message: "Sign the transfer intent (no gas)" });
    const signature = await signTypedDataAsync(
      buildOrderIntentTypedData({
        chainId: p.srcChainId,
        spender: adapter,
        token,
        inputAmount: p.amount,
        outputAmount,
        recipient,
        bridgeType: p.bridgeType,
        dstChainId: p.dstChainId,
        deliveryWindow: p.deliveryWindow,
        discountRate: p.discountRate,
        baseFee: p.baseFee,
        bridgeParams,
        hookDataHash,
        callbackGasLimit: p.callbackGasLimit,
        nonce,
        deadline,
      }),
    );

    setState({ phase: "submitting", message: "Relayer is submitting on-chain" });
    const res = await api.submitGasless({
      bridgeType: p.bridgeType,
      token: p.token,
      srcChainId: p.srcChainId,
      dstChainId: p.dstChainId,
      from,
      recipient,
      inputAmount: p.amount.toString(),
      outputAmount: outputAmount.toString(),
      deliveryWindow: Number(p.deliveryWindow),
      discountRate: p.discountRate.toString(),
      baseFee: p.baseFee.toString(),
      forwarding,
      hookData: p.hookData,
      callbackGasLimit: p.callbackGasLimit.toString(),
      maxFee: p.maxFee.toString(),
      minFinalityThreshold: p.minFinalityThreshold,
      lzReceiveGas: DEFAULT_LZRECEIVE_GAS.toString(),
      composeGas: DEFAULT_COMPOSE_GAS.toString(),
      permit: { nonce: nonce.toString(), deadline: deadline.toString(), signature },
    });

    setState({ phase: "done", message: "Submitted" });
    return { orderId: res.orderId, srcTxHash: res.srcTxHash };
  }

  async function submit(p: BridgeParams, mode: SubmitMode, forwarding: boolean): Promise<InitiateResult> {
    try {
      if (mode === "standard") return await runStandard(p, forwarding);
      if (mode === "permit2612") return await runPermit2612(p, forwarding);
      return await runPermit2(p, forwarding);
    } catch (e) {
      setState({ phase: "error", error: e instanceof Error ? e.message : String(e) });
      throw e;
    }
  }

  return { submit, state, reset };
}
