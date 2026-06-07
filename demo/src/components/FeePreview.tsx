"use client";

import { formatUnits } from "viem";
import type { PricingQuoteResponse } from "@/lib/api";
import { fmtAmount } from "@/lib/format";
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
          <span className="text-slate-400">Capital opportunity cost</span>
          <span className="font-mono text-slate-200">
            {fmtAmount(timeFee, decimals, 4)} {symbol}
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
            ≈ {fmtAmount(perMinute, decimals, 4)} {symbol}
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
      {comparison && (
        <div className="mt-2 rounded-md border border-edge bg-panel/50 p-2 text-[11px]">
          <div className="mb-1 text-slate-400">CCTP direct comparison</div>
          <div className="flex items-center justify-between">
            <span className="text-slate-500">Circle fast + forwarding</span>
            <span className="font-mono text-slate-200">
              {fmtAmount(BigInt(comparison.circleFastForwarding), decimals, 4)} {symbol}
            </span>
          </div>
          <div className="mt-0.5 flex items-center justify-between">
            <span className="text-slate-500">Circle slow + forwarding</span>
            <span className="font-mono text-slate-200">
              {fmtAmount(BigInt(comparison.circleSlowForwarding), decimals, 4)} {symbol}
            </span>
          </div>
          <div className="mt-0.5 flex items-center justify-between">
            <span className="text-slate-500">fast-fill estimate</span>
            <span className="font-mono text-slate-200">
              {fmtAmount(BigInt(comparison.fastFillEstimated), decimals, 4)} {symbol}
            </span>
          </div>
          <div className="mt-1 text-slate-500">
            vs fast forwarding:{" "}
            <span className={BigInt(comparison.savingsVsFastForwarding) >= 0n ? "text-good" : "text-warn"}>
              {BigInt(comparison.savingsVsFastForwarding) >= 0n ? "saves" : "costs"}{" "}
              {fmtAmount(abs(BigInt(comparison.savingsVsFastForwarding)), decimals, 4)} {symbol}
            </span>
          </div>
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
        mintFee={mintFee}
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
