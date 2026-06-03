"use client";

import { formatUnits } from "viem";
import {
  Area,
  AreaChart,
  CartesianGrid,
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
 */
export function PayoutCurve({
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
  const lo = data[0].amount; // instant-fill payout (the lowest point)
  const pad = full > lo ? (full - lo) * 0.2 : Math.max(full * 0.001, 1e-6);
  const yDomain: [number, number] = [Math.max(0, lo - pad), full + pad];

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
            <ReferenceLine
              y={full}
              stroke="#3ddc97"
              strokeDasharray="4 3"
              strokeOpacity={0.7}
              label={{
                value: `full ${fmtAmt(full)}`,
                position: "insideTopRight",
                fill: "#3ddc97",
                fontSize: 10,
              }}
            />
            <Area
              type="monotone"
              dataKey="amount"
              stroke="#5b8cff"
              strokeWidth={2}
              fill="url(#payoutFill)"
              dot={false}
              isAnimationActive={false}
            />
          </AreaChart>
        </ResponsiveContainer>
      </div>
    </div>
  );
}
