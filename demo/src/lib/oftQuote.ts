import type { Address, Hex } from "viem";
import { buildOrder, outputAmountOf, type BridgeParams } from "./bridge";
import { REGISTRY, type SupportedChainId } from "./chains";
import { adapterAddress } from "./config";
import { buildExtraOptions } from "./lzOptions";
import { oftAbi } from "./abis/oft";
import { addressToBytes32, encodeOrder } from "./order";
import { publicClientFor } from "./viemClients";

/**
 * Quote the LayerZero v2 native messaging fee (in wei) the sender must pay to send an OFT transfer.
 * This is the `nativeFee` returned by `quoteSend` — the gas the LZ executor is paid (in ETH) to
 * deliver the message and run the destination mint + compose (settlement). It is independent of the
 * connected wallet, so it can be quoted for a live fee preview as well as at submit time.
 *
 * `startTime` does not affect the fee (it only changes a few bytes of the fixed-size composeMsg), so
 * previews can pass any value; the authoritative fee is re-quoted in `useInitiate` before sending.
 */
export async function quoteOftNativeFee(p: BridgeParams, sender: Address, startTime = 0n): Promise<bigint> {
  const client = publicClientFor(p.srcChainId);
  const oft = REGISTRY[p.srcChainId].usdt0Oft!;
  const adapter = adapterAddress(p.bridgeType);
  const preview = buildOrder(p, sender, { startTime });
  const sendParam = {
    dstEid: REGISTRY[p.dstChainId as SupportedChainId].lzEid,
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
  return fee.nativeFee;
}
