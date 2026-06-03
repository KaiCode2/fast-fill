"use client";

import { fmtAmount } from "@/lib/format";
import { DEFAULT_MAX_FEE_RATE, feeOf, WAD } from "@/lib/pricing";

/**
 * Shows what the recipient receives on an *instant* fill (max premium, fillTime == startTime) and
 * notes that the premium decays to just the flat baseFee over the delivery window. Uses the local
 * PricingLib mirror; the on-chain `quoteFill` is authoritative at fill time.
 */
export function FeePreview({
  outputAmount,
  deliveryWindow,
  discountRate,
  baseFee,
  decimals,
  symbol,
  maxFeeRate = DEFAULT_MAX_FEE_RATE,
}: {
  outputAmount: bigint;
  deliveryWindow: bigint;
  discountRate: bigint;
  baseFee: bigint;
  decimals: number;
  symbol: string;
  maxFeeRate?: bigint;
}) {
  if (outputAmount <= 0n) return null;

  const instantFee = feeOf({
    outputAmount,
    startTime: 0n,
    expectedDeliveryTime: deliveryWindow,
    fillTime: 0n,
    discountRate,
    maxFeeRate,
    baseFee,
  });
  const instantPayout = outputAmount - instantFee;
  const ratePct = Number((instantFee * 1_000_000n) / outputAmount) / 10_000; // % with 4dp

  return (
    <div className="rounded-lg border border-edge bg-ink/60 p-3 text-sm">
      <div className="flex items-center justify-between">
        <span className="text-slate-400">Recipient receives (instant fill)</span>
        <span className="font-mono text-good">
          {fmtAmount(instantPayout, decimals, 4)} {symbol}
        </span>
      </div>
      <div className="mt-1 flex items-center justify-between">
        <span className="text-slate-400">Relayer premium</span>
        <span className="font-mono text-slate-200">
          {fmtAmount(instantFee, decimals, 4)} {symbol}{" "}
          <span className="text-slate-500">({ratePct.toLocaleString(undefined, { maximumFractionDigits: 3 })}%)</span>
        </span>
      </div>
      <p className="mt-2 text-[11px] leading-relaxed text-slate-500">
        Premium decays linearly to{" "}
        <span className="text-slate-300">
          {fmtAmount(baseFee, decimals, 4)} {symbol}
        </span>{" "}
        (the flat base fee) over {deliveryWindow.toString()}s. If no relayer fills, the recipient gets
        the full amount when the bridge settles — at no premium.
      </p>
    </div>
  );
}
