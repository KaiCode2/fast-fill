"use client";

import { formatUnits } from "viem";
import {
  Area,
  AreaChart,
  CartesianGrid,
  ReferenceDot,
  ReferenceLine,
  ResponsiveContainer,
  Tooltip,
  XAxis,
  YAxis,
} from "recharts";
import { DEFAULT_MAX_FEE_RATE, payoutOf } from "@/lib/pricing";

interface Point {
  t: number;
  amount: number;
}

/** "90s" / "10m" — compact axis/label time formatting from a seconds count. */
function fmtTime(s: number): string {
  if (s < 60) return `${Math.round(s)}s`;
  const m = s / 60;
  return `${m % 1 === 0 ? m : m.toFixed(1)}m`;
}

const fmtAmt = (v: number) => v.toLocaleString(undefined, { maximumFractionDigits: 4 });

function PayoutTooltip({
  active,
  payload,
  symbol,
}: {
  active?: boolean;
  payload?: { payload: Point }[];
  symbol: string;
}) {
  if (!active || !payload?.length) return null;
  const p = payload[0].payload;
  return (
    <div className="rounded-md border border-edge bg-panel px-2 py-1 text-[11px] shadow-lg">
      <div className="text-slate-400">fill within {fmtTime(p.t)}</div>
      <div className="font-mono text-good">
        {fmtAmt(p.amount)} {symbol}
      </div>
    </div>
  );
}

/**
 * Plots what the recipient receives as a function of how soon a relayer fills: the time-premium
 * decays from its instant maximum down to the flat baseFee across the delivery window, so the
 * payout *rises* over time toward the full amount. A dashed reference line marks the full amount
 * the recipient is guaranteed when the bridge itself settles (no premium) if nobody fast-fills.
 * Mirrors the on-chain `PricingLib`; `quoteFill` is authoritative at fill time.
 *
 * `cctpDirectReceived` (CCTP only) overlays what the recipient would net using Circle directly — a
 * single amber benchmark anchored at the settlement time (the right edge), since CCTP direct only
 * delivers once the bridge settles. The fast-fill curve sitting left-of and (usually) above it
 * shows the recipient gets funds earlier, and the vertical gap is the saving at any fill time.
 */
export function PayoutCurve({
  outputAmount,
  deliveryWindow,
  discountRate,
  baseFee,
  decimals,
  symbol,
  maxFeeRate = DEFAULT_MAX_FEE_RATE,
  mintFee = 0n,
  cctpDirectReceived,
}: {
  outputAmount: bigint;
  deliveryWindow: bigint;
  discountRate: bigint;
  baseFee: bigint;
  decimals: number;
  symbol: string;
  maxFeeRate?: bigint;
  mintFee?: bigint;
  cctpDirectReceived?: bigint;
}) {
  const windowSecs = Number(deliveryWindow);
  if (outputAmount <= 0n || windowSecs <= 0) return null;

  const toNum = (v: bigint) => Number(formatUnits(v, decimals));
  const POINTS = 41;
  const data: Point[] = Array.from({ length: POINTS }, (_, i) => {
    const t = Math.round((windowSecs * i) / (POINTS - 1));
    const payout = payoutOf({
      outputAmount,
      startTime: 0n,
      expectedDeliveryTime: deliveryWindow,
      fillTime: BigInt(t),
      discountRate,
      maxFeeRate,
      baseFee,
    });
    return { t, amount: toNum(payout) };
  });

  const full = toNum(outputAmount);
  const curveLo = data[0].amount; // instant-fill payout (the lowest point on the curve)
  // The flat CCTP executor fee is already netted out of `outputAmount`; `gross` is what would arrive
  // without it, so the gap between the two reference lines visualises the executor fee on the curve.
  const mintFeeNum = mintFee > 0n ? toNum(mintFee) : 0;
  const gross = full + mintFeeNum;
  const curveTop = mintFeeNum > 0 ? gross : full;
  // The CCTP-direct benchmark can fall outside the curve's range (its fees differ from fast-fill's),
  // so fold it into the y-domain to keep the marker on-chart.
  const cctpNum = cctpDirectReceived !== undefined ? toNum(cctpDirectReceived) : undefined;
  const lo = cctpNum !== undefined ? Math.min(curveLo, cctpNum) : curveLo;
  const top = cctpNum !== undefined ? Math.max(curveTop, cctpNum) : curveTop;
  const pad = top > lo ? (top - lo) * 0.2 : Math.max(top * 0.001, 1e-6);
  const yDomain: [number, number] = [Math.max(0, lo - pad), top + pad];

  return (
    <div className="mt-3">
      <div className="mb-1 flex items-center justify-between text-[11px] text-slate-500">
        <span>Recipient receives vs. time to fill</span>
        <span className="text-slate-400">guaranteed full by ~{fmtTime(windowSecs)}</span>
      </div>
      <div className="h-36 w-full">
        <ResponsiveContainer width="100%" height="100%">
          <AreaChart data={data} margin={{ top: 6, right: 8, bottom: 2, left: -12 }}>
            <defs>
              <linearGradient id="payoutFill" x1="0" y1="0" x2="0" y2="1">
                <stop offset="0%" stopColor="#5b8cff" stopOpacity={0.35} />
                <stop offset="100%" stopColor="#5b8cff" stopOpacity={0.02} />
              </linearGradient>
            </defs>
            <CartesianGrid stroke="#1e2430" vertical={false} />
            <XAxis
              dataKey="t"
              type="number"
              domain={[0, windowSecs]}
              tickFormatter={fmtTime}
              tick={{ fill: "#64748b", fontSize: 10 }}
              stroke="#1e2430"
            />
            <YAxis
              domain={yDomain}
              tickFormatter={fmtAmt}
              tick={{ fill: "#64748b", fontSize: 10 }}
              width={56}
              stroke="#1e2430"
            />
            <Tooltip content={<PayoutTooltip symbol={symbol} />} />
            {mintFeeNum > 0 && (
              <ReferenceLine
                y={gross}
                stroke="#94a3b8"
                strokeDasharray="2 3"
                strokeOpacity={0.55}
                label={{
                  value: `mint fee −${fmtAmt(mintFeeNum)}`,
                  position: "insideTopRight",
                  fill: "#94a3b8",
                  fontSize: 10,
                }}
              />
            )}
            <ReferenceLine
              y={full}
              stroke="#3ddc97"
              strokeDasharray="4 3"
              strokeOpacity={0.7}
              label={{
                value: `full ${fmtAmt(full)}`,
                position: mintFeeNum > 0 ? "insideBottomRight" : "insideTopRight",
                fill: "#3ddc97",
                fontSize: 10,
              }}
            />
            {cctpNum !== undefined && (
              <ReferenceLine
                y={cctpNum}
                stroke="#ffb454"
                strokeDasharray="5 3"
                strokeOpacity={0.75}
                label={{
                  value: `CCTP direct ${fmtAmt(cctpNum)}`,
                  position: "insideTopLeft",
                  fill: "#ffb454",
                  fontSize: 10,
                }}
              />
            )}
            <Area
              type="monotone"
              dataKey="amount"
              stroke="#5b8cff"
              strokeWidth={2}
              fill="url(#payoutFill)"
              dot={false}
              isAnimationActive={false}
            />
            {cctpNum !== undefined && (
              // Anchored at the right edge: CCTP direct only pays out once the bridge settles.
              <ReferenceDot
                x={windowSecs}
                y={cctpNum}
                r={4}
                fill="#ffb454"
                stroke="#0b0e14"
                strokeWidth={1.5}
                ifOverflow="visible"
              />
            )}
          </AreaChart>
        </ResponsiveContainer>
      </div>
    </div>
  );
}
