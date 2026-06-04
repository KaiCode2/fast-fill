"use client";

import { formatUnits } from "viem";
import { fmtAmount } from "@/lib/format";
import { DEFAULT_MAX_FEE_RATE, feeOf } from "@/lib/pricing";
import { PayoutCurve } from "./PayoutCurve";
import { DocLink } from "./docs/DocLink";
import { InfoTip } from "./InfoTip";

/** Compact ETH amount (wei → trimmed decimal string) for the LayerZero messaging fee. */
function fmtEth(wei: bigint): string {
  return Number(formatUnits(wei, 18)).toLocaleString(undefined, { maximumFractionDigits: 7 });
}

/**
 * Shows what the recipient receives on an *instant* fill (max premium, fillTime == startTime) and
 * notes that the premium decays to just the flat baseFee over the delivery window. Uses the local
 * PricingLib mirror; the on-chain `quoteFill` is authoritative at fill time.
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
}) {
  if (outputAmount <= 0n) return null;

  const isOft = lzFeeWei !== undefined;

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
      {mintFee > 0n && (
        <div className="mt-1 flex items-center justify-between">
          <span className="flex items-center gap-1 text-slate-400">
            CCTP executor fee
            <InfoTip label="What is the CCTP executor fee?">
              Flat fee paid to the CCTP executor contract so the relayer guarantees delivery — minted
              to the recipient even if no optimistic filler shows up.
            </InfoTip>
          </span>
          <span className="font-mono text-slate-200">
            {fmtAmount(mintFee, decimals, 4)} {symbol}
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
