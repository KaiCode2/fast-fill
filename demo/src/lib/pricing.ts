/**
 * Off-chain mirror of `src/libraries/PricingLib.sol` for instant UI preview. The authoritative
 * figure always comes from the on-chain `quoteFill(order, fillTime)` view; this is the
 * zero-latency local estimate used to render the decaying premium between RPC ticks.
 *
 *   timeSaved = max(0, expectedDeliveryTime - max(fillTime, startTime))
 *   rate      = min(discountRate * timeSaved, maxFeeRate)            [WAD]
 *   timeFee   = outputAmount * rate / WAD
 *   fee       = min(baseFee + timeFee, outputAmount)
 *   payout    = outputAmount - fee
 */

export const WAD = 10n ** 18n;

/** Default per-adapter governance cap on the time-premium rate (0.5%), per the deploy script. */
export const DEFAULT_MAX_FEE_RATE = 5n * 10n ** 15n;

export function feeOf(args: {
  outputAmount: bigint;
  startTime: bigint;
  expectedDeliveryTime: bigint;
  fillTime: bigint;
  discountRate: bigint;
  maxFeeRate: bigint;
  baseFee: bigint;
}): bigint {
  const { outputAmount, startTime, expectedDeliveryTime, fillTime, discountRate, maxFeeRate, baseFee } = args;
  const effectiveFill = fillTime < startTime ? startTime : fillTime;

  let timeFee = 0n;
  if (effectiveFill < expectedDeliveryTime) {
    const timeSaved = expectedDeliveryTime - effectiveFill; // > 0
    // bigint is arbitrary precision, so the Solidity saturating-product guard collapses to a
    // plain product followed by the maxFeeRate cap (an overflowing product is always > the cap).
    let rate = discountRate * timeSaved;
    if (rate > maxFeeRate) rate = maxFeeRate;
    timeFee = (outputAmount * rate) / WAD;
  }

  const total = baseFee + timeFee;
  return total > outputAmount ? outputAmount : total;
}

export function payoutOf(args: Parameters<typeof feeOf>[0]): bigint {
  return args.outputAmount - feeOf(args);
}
