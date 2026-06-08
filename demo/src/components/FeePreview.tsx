"use client";

import { formatUnits } from "viem";
import type { PricingQuoteResponse } from "@/lib/api";
import { fmtAmount, fmtAmountNonZero } from "@/lib/format";
import { EXPECTED_FILL_DELAY_SECS } from "@/lib/gasPricing";
import { DEFAULT_MAX_FEE_RATE, feeOf } from "@/lib/pricing";
import { PayoutCurve } from "./PayoutCurve";
import { DocLink } from "./docs/DocLink";
import { InfoTip } from "./InfoTip";

/** Compact ETH amount (wei → trimmed decimal string) for the LayerZero messaging fee. */
function fmtEth(wei: bigint): string {
  return Number(formatUnits(wei, 18)).toLocaleString(undefined, { maximumFractionDigits: 7 });
}

function fmtGwei(wei: string): string {
  return Number(formatUnits(BigInt(wei), 9)).toLocaleString(undefined, { maximumFractionDigits: 4 });
}

function abs(v: bigint): bigint {
  return v < 0n ? -v : v;
}

/** Compact seconds → "45s" / "19m" for the speed comparison. */
function fmtSecs(s: number): string {
  if (s < 90) return `${Math.round(s)}s`;
  const m = s / 60;
  return `${m % 1 === 0 ? m : m.toFixed(m < 10 ? 1 : 0)}m`;
}

/** Typical optimistic fill latency used purely for the "× faster than CCTP" framing. */
const FAST_FILL_SECS = 1.5;

/**
 * Shows what the recipient receives at the expected fast-fill delay and notes that the premium
 * decays to just the flat baseFee over the delivery window. Uses the local PricingLib mirror; the
 * on-chain `quoteFill` is authoritative at fill time.
 *
 * `mintFee` is the flat CCTP executor ("Relay Mint") fee — already deducted from `outputAmount` —
 * surfaced as its own line item and drawn on the curve. `lzFeeWei` is the LayerZero native send fee
 * (ETH gas) for OFT transfers; `undefined` means "not an OFT transfer" (line hidden).
 */
export function FeePreview({
  outputAmount,
  deliveryWindow,
  discountRate,
  baseFee,
  decimals,
  symbol,
  maxFeeRate = DEFAULT_MAX_FEE_RATE,
  mintFee = 0n,
  lzFeeWei,
  lzFeeLoading = false,
  pricingBreakdown,
  comparison,
  actionLabel,
}: {
  outputAmount: bigint;
  deliveryWindow: bigint;
  discountRate: bigint;
  baseFee: bigint;
  decimals: number;
  symbol: string;
  maxFeeRate?: bigint;
  mintFee?: bigint;
  lzFeeWei?: bigint | null;
  lzFeeLoading?: boolean;
  pricingBreakdown?: PricingQuoteResponse["breakdown"];
  comparison?: PricingQuoteResponse["comparison"];
  /** Short name of the destination action (e.g. "Aave deposit"), shown in the CCTP comparison. */
  actionLabel?: string;
}) {
  if (outputAmount <= 0n) return null;

  const isOft = lzFeeWei !== undefined;

  const expectedFillTime =
    deliveryWindow > EXPECTED_FILL_DELAY_SECS ? EXPECTED_FILL_DELAY_SECS : 0n;
  const expectedFee = feeOf({
    outputAmount,
    startTime: 0n,
    expectedDeliveryTime: deliveryWindow,
    fillTime: expectedFillTime,
    discountRate,
    maxFeeRate,
    baseFee,
  });
  const expectedPayout = outputAmount - expectedFee;
  const ratePct = Number((expectedFee * 1_000_000n) / outputAmount) / 10_000; // % with 4dp
  // The premium decays linearly to baseFee over the window, so the time-premium portion costs a
  // constant amount per minute the recipient receives sooner.
  const perMinute = deliveryWindow > 0n ? ((expectedFee - baseFee) * 60n) / deliveryWindow : 0n;
  const timeFee = expectedFee > baseFee ? expectedFee - baseFee : 0n;

  return (
    <div className="rounded-lg border border-edge bg-ink/60 p-3 text-sm">
      <div className="flex items-center justify-between">
        <span className="text-slate-400">Recipient receives (~15s fill)</span>
        <span className="font-mono text-good">
          {fmtAmount(expectedPayout, decimals, 4)} {symbol}
        </span>
      </div>
      <div className="mt-1 flex items-center justify-between">
        <span className="text-slate-400">Relayer premium</span>
        <span className="font-mono text-slate-200">
          {fmtAmount(expectedFee, decimals, 4)} {symbol}{" "}
          <span className="text-slate-500">({ratePct.toLocaleString(undefined, { maximumFractionDigits: 3 })}%)</span>
        </span>
      </div>
      {mintFee > 0n && (
        <div className="mt-1 flex items-center justify-between">
          <span className="flex items-center gap-1 text-slate-400">
            Relay Mint gas fee
            <InfoTip label="What is the Relay Mint gas fee?">
              Fee paid to whoever executes the CCTP mint through the executor. It is quoted from the
              destination gas price with a 25% buffer.
            </InfoTip>
          </span>
          <span className="font-mono text-slate-200">
            {fmtAmount(mintFee, decimals, 4)} {symbol}
          </span>
        </div>
      )}
      <div className="mt-1 flex items-center justify-between">
        <span className="flex items-center gap-1 text-slate-400">
          Gas-backed base fee
          <InfoTip label="What is the gas-backed base fee?">
            The relayer&apos;s destination gas cost for filling this order, using the gas benchmarks,
            live destination gas price, ETH/USD, and a 25% buffer.
          </InfoTip>
        </span>
        <span className="font-mono text-slate-200">
          {fmtAmount(baseFee, decimals, 4)} {symbol}
        </span>
      </div>
      {timeFee > 0n && (
        <div className="mt-1 flex items-center justify-between">
          <span className="flex items-center gap-1 text-slate-400">
            Capital opportunity cost
            <InfoTip label="What is the capital opportunity cost?">
              The relayer fronts the funds before the bridge settles, so it charges the time value of
              that capital (≈10% APR) for the seconds you receive sooner. Tiny, but never zero.
            </InfoTip>
          </span>
          <span className="font-mono text-slate-200">
            {fmtAmountNonZero(timeFee, decimals, 4)} {symbol}
          </span>
        </div>
      )}
      {isOft && (
        <div className="mt-1 flex items-center justify-between">
          <span className="flex items-center gap-1 text-slate-400">
            LayerZero send fee
            <InfoTip label="What is the LayerZero send fee?">
              Network fee (paid in ETH) for the LayerZero message that carries the OFT transfer and
              triggers settlement on the destination. Paid on top of the bridged amount.
            </InfoTip>
          </span>
          <span className="font-mono text-slate-200">
            {lzFeeWei != null ? (
              <>≈ {fmtEth(lzFeeWei)} ETH</>
            ) : (
              <span className="text-slate-500">{lzFeeLoading ? "quoting…" : "—"}</span>
            )}
          </span>
        </div>
      )}
      {perMinute > 0n && (
        <div className="mt-1 flex items-center justify-between">
          <span className="text-slate-400">Cost of speed</span>
          <span className="font-mono text-slate-200">
            ≈ {fmtAmountNonZero(perMinute, decimals, 4)} {symbol}
            <span className="text-slate-500">/min saved</span>
          </span>
        </div>
      )}
      {pricingBreakdown && (
        <div className="mt-2 rounded-md border border-edge bg-panel/50 px-2 py-1 text-[11px] text-slate-500">
          <div className="flex items-center justify-between">
            <span>Gas quote</span>
            <span className="font-mono">
              {fmtGwei(pricingBreakdown.gasPriceWei)} gwei · ETH ${Number(pricingBreakdown.ethUsd).toLocaleString()}
            </span>
          </div>
          <div className="mt-0.5 flex items-center justify-between">
            <span>Fill budget</span>
            <span className="font-mono">{Number(pricingBreakdown.fillGasUnits).toLocaleString()} gas</span>
          </div>
        </div>
      )}
      {comparison && (() => {
        const savings = BigInt(comparison.savings);
        const circleDirect = BigInt(comparison.circleDirect);
        const cheaper = savings >= 0n;
        // Percentage saved (or overpaid) relative to going via Circle directly.
        const pct = circleDirect > 0n ? Number((abs(savings) * 10_000n) / circleDirect) / 100 : null;
        const cctpSecs = Number(deliveryWindow);
        const speedup = cctpSecs > FAST_FILL_SECS ? Math.round(cctpSecs / FAST_FILL_SECS) : 0;
        // Destination action (Aave deposit / Uniswap swap): fast-fill runs it atomically in the fill,
        // so going direct adds a separate post-settlement transaction to the cost + steps.
        const hasAction = BigInt(comparison.circleExecGas) > 0n;
        const action = actionLabel ?? "execution";
        const steps = [comparison.forwarding ? "forwarding" : "self-claim"];
        if (hasAction) steps.push(action);
        return (
          <div className="mt-2 rounded-md border border-edge bg-panel/50 p-2 text-[11px]">
            <div className="mb-1 flex items-center gap-1 text-slate-400">
              CCTP direct comparison
              <InfoTip label="How does this compare to CCTP direct?">
                What this transfer would cost and how long it would take using Circle CCTP directly,
                with the same settings.{" "}
                {comparison.forwarding
                  ? "Includes Circle's forwarding fee for auto-delivery."
                  : "Includes the destination gas you'd pay to mint (claim) the USDC yourself."}
                {hasAction
                  ? ` fast-fill runs the ${action} atomically in the fill; via Circle you'd pay for a separate ${action} transaction afterwards.`
                  : ""}
              </InfoTip>
            </div>
            <div className="flex items-center justify-between">
              <span className="text-slate-500">
                Circle {comparison.speed} ({steps.join(" + ")})
              </span>
              <span className="font-mono text-slate-200">
                {fmtAmountNonZero(circleDirect, decimals, 4)} {symbol}
              </span>
            </div>
            <div className="mt-0.5 flex items-center justify-between">
              <span className="text-slate-500">fast-fill estimate{hasAction ? " (one-shot)" : ""}</span>
              <span className="font-mono text-slate-200">
                {fmtAmountNonZero(BigInt(comparison.fastFillEstimated), decimals, 4)} {symbol}
              </span>
            </div>
            <div className="mt-1 flex items-center justify-between">
              <span className="text-slate-500">Cost</span>
              <span className={cheaper ? "text-good" : "text-warn"}>
                {cheaper ? "saves" : "costs"} {fmtAmountNonZero(abs(savings), decimals, 4)} {symbol}
                {pct !== null && (
                  <span className="text-slate-500">
                    {" "}({pct.toLocaleString(undefined, { maximumFractionDigits: 1 })}%{" "}
                    {cheaper ? "cheaper" : "more"})
                  </span>
                )}
              </span>
            </div>
            <div className="mt-0.5 flex items-center justify-between">
              <span className="text-slate-500">Speed</span>
              <span className="text-slate-200">
                ~{FAST_FILL_SECS}s{hasAction ? " one-shot" : ""} vs ~{fmtSecs(cctpSecs)}
                {hasAction ? " + a separate tx" : ""}
                {speedup > 1 && <span className="text-good"> (≈{speedup.toLocaleString()}× faster)</span>}
              </span>
            </div>
          </div>
        );
      })()}
      <PayoutCurve
        outputAmount={outputAmount}
        deliveryWindow={deliveryWindow}
        discountRate={discountRate}
        baseFee={baseFee}
        decimals={decimals}
        symbol={symbol}
        maxFeeRate={maxFeeRate}
        mintFee={mintFee}
        cctpDirectReceived={comparison ? BigInt(comparison.cctpDirectReceived) : undefined}
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
      {comparison && (
        <p className="mt-1 text-[11px] leading-relaxed text-slate-500">
          The{" "}
          <span className="text-warn">amber</span> marker is CCTP direct: the recipient would net{" "}
          <span className="text-warn">
            {fmtAmountNonZero(BigInt(comparison.cctpDirectReceived), decimals, 4)} {symbol}
          </span>{" "}
          only once the bridge settles (~{fmtSecs(Number(deliveryWindow))}) — see the cost and speed
          comparison above.
        </p>
      )}
      <p className="mt-2 text-[11px]">
        <DocLink href="/docs/architecture#5-pricing">How pricing works</DocLink>
      </p>
    </div>
  );
}
