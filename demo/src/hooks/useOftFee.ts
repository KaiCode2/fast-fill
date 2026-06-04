"use client";

import { useEffect, useState } from "react";
import type { Address } from "viem";
import { useAccount } from "wagmi";
import type { BridgeParams } from "@/lib/bridge";
import { quoteOftNativeFee } from "@/lib/oftQuote";
import { BRIDGE_OFT } from "@/lib/order";

/**
 * Live LayerZero native-fee quote (wei) for the current OFT params, for fee preview. Debounced and
 * keyed on the inputs that actually move the fee (route + amount); returns null for non-OFT params
 * or while a quote is in flight/failed. The authoritative fee is re-quoted at submit time.
 */
export function useOftFee(params: BridgeParams | null): { fee: bigint | null; loading: boolean } {
  const { address } = useAccount();
  const [fee, setFee] = useState<bigint | null>(null);
  const [loading, setLoading] = useState(false);

  const active = Boolean(params && params.bridgeType === BRIDGE_OFT && params.amount > 0n && address);
  // Re-quote only when a fee-relevant input changes — not on every keystroke elsewhere in the form.
  const key = active ? `${params!.srcChainId}-${params!.dstChainId}-${params!.amount}-${address}` : "";

  useEffect(() => {
    if (!active) {
      setFee(null);
      setLoading(false);
      return;
    }
    let cancelled = false;
    setLoading(true);
    const handle = setTimeout(() => {
      quoteOftNativeFee(params!, address as Address)
        .then((f) => !cancelled && setFee(f))
        .catch(() => !cancelled && setFee(null))
        .finally(() => !cancelled && setLoading(false));
    }, 350);
    return () => {
      cancelled = true;
      clearTimeout(handle);
    };
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, [key]);

  return { fee, loading };
}
