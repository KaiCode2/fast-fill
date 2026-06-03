import { formatUnits, type Address } from "viem";

/** Human-friendly token amount (base units → decimal string with thousands separators). */
export function fmtAmount(amount: bigint | undefined, decimals: number, dp = 4): string {
  if (amount === undefined) return "—";
  const n = Number(formatUnits(amount, decimals));
  return n.toLocaleString(undefined, { maximumFractionDigits: dp, minimumFractionDigits: 0 });
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
