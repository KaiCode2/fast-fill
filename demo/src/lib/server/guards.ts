import "server-only";
import { isSupportedChain, type SupportedChainId } from "@/lib/chains";
import { bytes32ToAddress, type Order } from "@/lib/order";
import { isRouteSupported, symbolByAddress } from "@/lib/tokens";
import { MAX_TRANSFER_BASE_UNITS } from "./env";

/** Reject anything outside the demo's allowed routes/tokens/caps before the relayer touches it. */
export function assertOrderAllowed(order: Order): void {
  if (!isSupportedChain(order.srcChainId) || !isSupportedChain(order.dstChainId)) {
    throw new Error("unsupported chain");
  }
  const src = order.srcChainId as SupportedChainId;
  const dst = order.dstChainId as SupportedChainId;

  const inSym = symbolByAddress(src, bytes32ToAddress(order.inputToken));
  const outSym = symbolByAddress(dst, bytes32ToAddress(order.outputToken));
  if (!inSym) throw new Error("unknown input token");
  if (outSym !== inSym) throw new Error("input/output token mismatch");
  if (!isRouteSupported(inSym, src, dst)) throw new Error("route not allowed");

  if (order.inputAmount <= 0n) throw new Error("invalid amount");
  if (order.inputAmount > MAX_TRANSFER_BASE_UNITS) {
    throw new Error(`amount exceeds demo cap (${MAX_TRANSFER_BASE_UNITS} base units)`);
  }
  if (order.outputAmount <= 0n || order.outputAmount > order.inputAmount) {
    throw new Error("invalid output amount");
  }
}

/** Minimal in-memory token bucket per key (IP). */
const buckets = new Map<string, { count: number; reset: number }>();
export function rateLimit(key: string, limit = 20, windowMs = 60_000): boolean {
  const now = Date.now();
  const b = buckets.get(key);
  if (!b || now > b.reset) {
    buckets.set(key, { count: 1, reset: now + windowMs });
    return true;
  }
  if (b.count >= limit) return false;
  b.count += 1;
  return true;
}
