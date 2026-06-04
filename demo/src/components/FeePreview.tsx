"use client";

import { fmtAmount } from "@/lib/format";
import { DEFAULT_MAX_FEE_RATE, feeOf } from "@/lib/pricing";
import { PayoutCurve } from "./PayoutCurve";
import { DocLink } from "./docs/DocLink";

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
  // The premium decays linearly to baseFee over the window, so the time-premium portion costs a
  // constant amount per minute the recipient receives sooner.
  const perMinute = deliveryWindow > 0n ? ((instantFee - baseFee) * 60n) / deliveryWindow : 0n;

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
      {perMinute > 0n && (
        <div className="mt-1 flex items-center justify-between">
          <span className="text-slate-400">Cost of speed</span>
          <span className="font-mono text-slate-200">
            ≈ {fmtAmount(perMinute, decimals, 4)} {symbol}
            <span className="text-slate-500">/min saved</span>
          </span>
        </div>
      )}
      <PayoutCurve
        outputAmount={outputAmount}
        deliveryWindow={deliveryWindow}
        discountRate={discountRate}
        baseFee={baseFee}
        decimals={decimals}
        symbol={symbol}
        maxFeeRate={maxFeeRate}
      />
      <p className="mt-2 text-[11px] leading-relaxed text-slate-500">
        The premium decays linearly to{" "}
        <span className="text-slate-300">
          {fmtAmount(baseFee, decimals, 4)} {symbol}
        </span>{" "}
        (the flat base fee) over {deliveryWindow.toString()}s — so the longer the recipient waits, the
        more they receive. If no relayer fills, the recipient is guaranteed the full{" "}
        <span className="text-good">
          {fmtAmount(outputAmount, decimals, 4)} {symbol}
        </span>{" "}
        when the bridge settles (~{deliveryWindow.toString()}s) — at no premium.
      </p>
      <p className="mt-2 text-[11px]">
        <DocLink href="/docs/architecture#5-pricing">How pricing works</DocLink>
      </p>
    </div>
  );
}
