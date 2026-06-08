"use client";

import { useCallback, useEffect, useState } from "react";
import type { Hex } from "viem";
import type { SerializedOrder, SubmitMode } from "@/lib/api";
import type { SupportedChainId, TokenSymbol } from "@/lib/chains";
import type { HookKind } from "@/lib/hooks";

/** A transfer the user started, persisted client-side so history survives reloads. */
export interface TransferRecord {
  orderId: Hex;
  mode: SubmitMode;
  bridgeType: number;
  token: TokenSymbol;
  srcChainId: SupportedChainId;
  dstChainId: SupportedChainId;
  amount: string; // base units
  outputAmount: string;
  recipient: Hex; // beneficiary address (with a hook, the on-chain recipient is the hook contract)
  srcTxHash?: Hex;
  forwarding: boolean;
  hookKind?: HookKind; // destination action attached to this transfer
  swapTokenSymbol?: string; // for "uniswap": the token swapped into
  order?: SerializedOrder; // full on-chain Order (when captured) — enables client-side self-relay
  createdAt: number;
}

const KEY = "fastfill.transfers.v1";

function load(): TransferRecord[] {
  if (typeof window === "undefined") return [];
  try {
    const raw = window.localStorage.getItem(KEY);
    return raw ? (JSON.parse(raw) as TransferRecord[]) : [];
  } catch {
    return [];
  }
}

export function useTransfers() {
  const [transfers, setTransfers] = useState<TransferRecord[]>([]);

  useEffect(() => {
    setTransfers(load());
    const onStorage = (e: StorageEvent) => {
      if (e.key === KEY) setTransfers(load());
    };
    window.addEventListener("storage", onStorage);
    return () => window.removeEventListener("storage", onStorage);
  }, []);

  const persist = useCallback((next: TransferRecord[]) => {
    setTransfers(next);
    try {
      window.localStorage.setItem(KEY, JSON.stringify(next));
    } catch {
      /* ignore quota errors in the demo */
    }
  }, []);

  const add = useCallback(
    (t: TransferRecord) => persist([t, ...load().filter((x) => x.orderId !== t.orderId)].slice(0, 30)),
    [persist],
  );

  const update = useCallback(
    (orderId: Hex, patch: Partial<TransferRecord>) =>
      persist(load().map((t) => (t.orderId === orderId ? { ...t, ...patch } : t))),
    [persist],
  );

  return { transfers, add, update };
}
