"use client";

import { REGISTRY } from "@/lib/chains";
import { fmtAmount } from "@/lib/format";
import type { TransferRecord } from "@/hooks/useTransfers";
import { OrderTimeline } from "./OrderTimeline";

export function TransferHistory({ transfers }: { transfers: TransferRecord[] }) {
  return (
    <div className="card">
      <h2 className="mb-3 text-sm font-semibold text-slate-100">Transfers</h2>
      {transfers.length === 0 ? (
        <p className="text-sm text-slate-500">
          No transfers yet — the ones you start will appear here and persist in this browser.
        </p>
      ) : (
        <div className="space-y-4">
          {transfers.map((t) => (
          <div key={t.orderId} className="rounded-lg border border-edge bg-ink/60 p-4">
            <div className="mb-3 flex items-center justify-between">
              <div className="text-sm text-slate-200">
                {fmtAmount(BigInt(t.amount), REGISTRY[t.srcChainId].usdc.decimals, 4)} {t.token}{" "}
                <span className="text-slate-500">
                  {REGISTRY[t.srcChainId].shortName} → {REGISTRY[t.dstChainId].shortName}
                </span>
                {t.hookKind && t.hookKind !== "none" && (
                  <span className="pill ml-2 border-accent/40 bg-accent/10 text-accent">
                    {t.hookKind === "aave" ? "→ Aave" : `→ Swap ${t.swapTokenSymbol ?? ""}`.trim()}
                  </span>
                )}
              </div>
              <span className="pill border-edge bg-panel text-slate-400">{t.mode}</span>
            </div>
            <OrderTimeline t={t} />
          </div>
          ))}
        </div>
      )}
    </div>
  );
}
