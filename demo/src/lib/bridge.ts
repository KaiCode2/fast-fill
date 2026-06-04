import type { Address, Hex } from "viem";
import { type SupportedChainId, type TokenSymbol } from "./chains";
import { addressToBytes32, BRIDGE_CCTP, type Order } from "./order";
import { getToken } from "./tokens";

/** CCTP minFinalityThreshold presets. */
export const FINALITY_FAST = 1000;
export const FINALITY_FINALIZED = 2000;

/**
 * Delivery-window presets — how long the *underlying* bridge takes to deliver to the recipient if
 * nobody fast-fills. The premium decays to zero across this window, so it is the time a fast fill
 * actually saves. Sourced from Circle's CCTP V2 finality docs for Arbitrum / OP / Base:
 *   - Fast (soft finality, minFinalityThreshold ≤ 1000): ~8s; we pad to ~1 min for attestation + mint.
 *   - Standard (hard finality, minFinalityThreshold ≥ 2000): ~15–19 min; we use 19 min.
 * LayerZero (USDT0/OFT) delivery is typically a couple of minutes.
 */
export const CCTP_FAST_DELIVERY_SECS = 60n;
export const CCTP_STANDARD_DELIVERY_SECS = 1140n; // 19 min
export const OFT_DELIVERY_SECS = 180n; // ~3 min

/** Default delivery window for a transfer: CCTP keys off fast/finalized; OFT is a flat estimate. */
export function deliveryWindowFor(bridgeType: number, finality: number): bigint {
  if (bridgeType !== BRIDGE_CCTP) return OFT_DELIVERY_SECS;
  return finality === FINALITY_FINALIZED ? CCTP_STANDARD_DELIVERY_SECS : CCTP_FAST_DELIVERY_SECS;
}

/** All the user-tunable inputs for one transfer. */
export interface BridgeParams {
  token: TokenSymbol;
  bridgeType: number;
  srcChainId: SupportedChainId;
  dstChainId: SupportedChainId;
  amount: bigint; // inputAmount, base units
  recipient: Address;
  maxFee: bigint; // CCTP fast-transfer fee cap (0 for OFT / finalized)
  mintFee: bigint; // CCTP executor relay incentive (0 = legacy direct settle path)
  minFinalityThreshold: number; // CCTP only
  deliveryWindow: bigint; // relative seconds
  discountRate: bigint; // WAD per second
  baseFee: bigint; // flat fee, output-token units
  callbackGasLimit: bigint; // gas for the recipient's onFastFill hook (0 = none)
  hookData: Hex; // destination-execution payload ("0x" = deliver funds only)
}

/** Worst-case amount the filler is owed: CCTP nets transport + mint relay fees, OFT is 1:1. */
export function outputAmountOf(p: Pick<BridgeParams, "bridgeType" | "amount" | "maxFee" | "mintFee">): bigint {
  return p.bridgeType === BRIDGE_CCTP ? p.amount - p.maxFee - p.mintFee : p.amount;
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

/** Args for `initiateCCTP(..., mintFee, minFinalityThreshold, ..., baseFee, exec)`. */
export function cctpInitiateArgs(p: BridgeParams) {
  return [
    p.dstChainId,
    addressToBytes32(p.recipient),
    p.amount,
    p.maxFee,
    p.mintFee,
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
