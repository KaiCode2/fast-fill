import "server-only";
import type { Hex } from "viem";
import { cctpAdapterAbi, oftAdapterAbi } from "@/lib/abis/generated";
import { erc20Abi } from "@/lib/abis/erc20";
import type { SupportedChainId } from "@/lib/chains";
import { bytes32ToAddress, BRIDGE_CCTP, type Order } from "@/lib/order";
import { pub, relayerAccount, wallet, withChainLock } from "./clients";
import { adapterAddressServer } from "./env";
import { requiredFillFeeForOrder } from "./pricing";

export type FillOutcome =
  | { kind: "filled"; fillTxHash: Hex; payout: bigint; fee: bigint }
  | { kind: "already" }
  | { kind: "insufficient"; have: bigint; need: bigint }
  | { kind: "skipped"; reason: string }
  | { kind: "error"; error: string };

const nowSec = () => BigInt(Math.floor(Date.now() / 1000));

/** Quote → liquidity check → gas-floor policy → simulate → fill. The relayer is the filler and the payer. */
export async function fillOrder(order: Order, bridgeType: number, mintFee = 0n): Promise<FillOutcome> {
  const dst = order.dstChainId as SupportedChainId;
  const adapter = adapterAddressServer(bridgeType);
  const abi = bridgeType === BRIDGE_CCTP ? cctpAdapterAbi : oftAdapterAbi;
  const tokenAddr = bytes32ToAddress(order.outputToken);
  const relayer = relayerAccount().address;

  // Liquidity: the relayer fronts the worst-case payout (outputAmount), reimbursed at settle.
  const balance = (await pub(dst).readContract({
    address: tokenAddr,
    abi: erc20Abi,
    functionName: "balanceOf",
    args: [relayer],
  })) as bigint;
  if (balance < order.outputAmount) {
    return { kind: "insufficient", have: balance, need: order.outputAmount };
  }

  const [payout, fee] = (await pub(dst).readContract({
    address: adapter,
    abi,
    functionName: "quoteFill",
    args: [order, nowSec()],
  })) as readonly [bigint, bigint];
  const requiredFee = await requiredFillFeeForOrder({
    bridgeType,
    dstChainId: dst,
    callbackGasLimit: order.callbackGasLimit,
    mintFee,
    decimals: 6,
  });
  if (fee < requiredFee) return { kind: "skipped", reason: `premium below gas floor (${fee} < ${requiredFee})` };

  try {
    const fillTxHash = await withChainLock(dst, async () => {
      const { request } = await pub(dst).simulateContract({
        account: relayerAccount(),
        address: adapter,
        abi,
        functionName: "fill",
        args: [order],
      });
      const hash = await wallet(dst).writeContract(request);
      await pub(dst).waitForTransactionReceipt({ hash });
      return hash;
    });
    return { kind: "filled", fillTxHash, payout, fee };
  } catch (e) {
    const msg = e instanceof Error ? e.message : String(e);
    if (/OrderAlreadyActive/i.test(msg)) return { kind: "already" };
    return { kind: "error", error: msg.slice(0, 200) };
  }
}
