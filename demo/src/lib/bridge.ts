import type { Address, Hex } from "viem";
import { type SupportedChainId, type TokenSymbol } from "./chains";
import { addressToBytes32, BRIDGE_CCTP, type Order } from "./order";
import { getToken } from "./tokens";

/** CCTP minFinalityThreshold presets. */
export const FINALITY_FAST = 1000;
export const FINALITY_FINALIZED = 2000;

/** All the user-tunable inputs for one transfer. */
export interface BridgeParams {
  token: TokenSymbol;
  bridgeType: number;
  srcChainId: SupportedChainId;
  dstChainId: SupportedChainId;
  amount: bigint; // inputAmount, base units
  recipient: Address;
  maxFee: bigint; // CCTP fast-transfer fee cap (0 for OFT / finalized)
  minFinalityThreshold: number; // CCTP only
  deliveryWindow: bigint; // relative seconds
  discountRate: bigint; // WAD per second
  baseFee: bigint; // flat fee, output-token units
  callbackGasLimit: bigint; // gas for the recipient's onFastFill hook (0 = none)
  hookData: Hex; // destination-execution payload ("0x" = deliver funds only)
}

/** Worst-case amount the filler is owed: CCTP nets the fee, OFT is 1:1 (minAmountLD). */
export function outputAmountOf(p: Pick<BridgeParams, "bridgeType" | "amount" | "maxFee">): bigint {
  return p.bridgeType === BRIDGE_CCTP ? p.amount - p.maxFee : p.amount;
}

/** A reasonable default CCTP fast-transfer fee cap (~2 bps), with a 1-unit floor. */
export function suggestMaxFee(amount: bigint): bigint {
  if (amount <= 0n) return 0n;
  const bps2 = amount / 5000n;
  return bps2 > 0n ? bps2 : 1n;
}

/**
 * Build a full Order for previewing the fee / reconstructing for the backend. `startTime` is set
 * to the source block timestamp (use `now` for previews; the real value is read from the mined
 * block for the handoff). `nonce` does not affect the fee, so 0 is fine for previews.
 */
export function buildOrder(
  p: BridgeParams,
  sender: Address,
  opts: { nonce?: bigint; startTime: bigint },
): Order {
  const srcTok = getToken(p.srcChainId, p.token);
  const dstTok = getToken(p.dstChainId, p.token);
  return {
    bridgeType: p.bridgeType,
    srcChainId: p.srcChainId,
    dstChainId: p.dstChainId,
    sender: addressToBytes32(sender),
    recipient: addressToBytes32(p.recipient),
    inputToken: addressToBytes32(srcTok.address),
    outputToken: addressToBytes32(dstTok.address),
    inputAmount: p.amount,
    outputAmount: outputAmountOf(p),
    nonce: opts.nonce ?? 0n,
    startTime: opts.startTime,
    expectedDeliveryTime: opts.startTime + p.deliveryWindow,
    discountRate: p.discountRate,
    baseFee: p.baseFee,
    callbackGasLimit: p.callbackGasLimit,
    hookData: p.hookData,
  };
}

/** The Execution struct (destination callback) every initiate entrypoint takes after `baseFee`. */
export function execArg(p: BridgeParams) {
  return { gasLimit: p.callbackGasLimit, data: p.hookData } as const;
}

/** Args for `initiateCCTP(..., baseFee, exec)`. */
export function cctpInitiateArgs(p: BridgeParams) {
  return [
    p.dstChainId,
    addressToBytes32(p.recipient),
    p.amount,
    p.maxFee,
    p.minFinalityThreshold,
    p.deliveryWindow,
    p.discountRate,
    p.baseFee,
    execArg(p),
  ] as const;
}

/** Args for `initiateOFT(..., baseFee, exec)`. */
export function oftInitiateArgs(p: BridgeParams, extraOptions: `0x${string}`) {
  return [
    p.dstChainId,
    addressToBytes32(p.recipient),
    p.amount,
    outputAmountOf(p), // minAmountLD == outputAmount (1:1 mint/burn)
    extraOptions,
    p.deliveryWindow,
    p.discountRate,
    p.baseFee,
    execArg(p),
  ] as const;
}
