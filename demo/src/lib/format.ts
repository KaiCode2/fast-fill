import { formatUnits, type Address } from "viem";

/** Human-friendly token amount (base units → decimal string with thousands separators). */
export function fmtAmount(amount: bigint | undefined, decimals: number, dp = 4): string {
  if (amount === undefined) return "—";
  const n = Number(formatUnits(amount, decimals));
  return n.toLocaleString(undefined, { maximumFractionDigits: dp, minimumFractionDigits: 0 });
}

/**
 * Like {@link fmtAmount} but never renders a positive amount as a flat "0": when the value would
 * round to zero at `dp`, precision is bumped (up to the token's own `decimals`) until a non-zero
 * digit appears. Used for tiny-but-real figures (e.g. the capital opportunity cost) so the UI never
 * implies a cost is absent when it is merely small.
 */
export function fmtAmountNonZero(amount: bigint, decimals: number, dp = 4): string {
  if (amount <= 0n) return fmtAmount(amount, decimals, dp);
  // toLocaleString rounds half away from zero, so the value shows as "0" iff 2·amount·10^p < 10^dec.
  const scale = 10n ** BigInt(decimals);
  let p = dp;
  while (p < decimals && 2n * amount * 10n ** BigInt(p) < scale) p++;
  return fmtAmount(amount, decimals, p);
}

/** Full-precision decimal string (no rounding) — for exact figures in the timeline. */
export function fmtExact(amount: bigint, decimals: number): string {
  return formatUnits(amount, decimals);
}

export function shortAddr(a?: string): string {
  return a ? `${a.slice(0, 6)}…${a.slice(-4)}` : "";
}

export function shortHash(h?: string): string {
  return h ? `${h.slice(0, 10)}…${h.slice(-6)}` : "";
}

export const ZERO_ADDRESS = "0x0000000000000000000000000000000000000000" as Address;
