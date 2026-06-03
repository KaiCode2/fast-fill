"use client";

import { REGISTRY } from "@/lib/chains";
import { fmtAmount } from "@/lib/format";
import type { TransferRecord } from "@/hooks/useTransfers";
import { OrderTimeline } from "./OrderTimeline";

export function TransferHistory({ transfers }: { transfers: TransferRecord[] }) {
  if (transfers.length === 0) return null;

  return (
    <div className="card">
      <h2 className="mb-3 text-sm font-semibold text-slate-100">Transfers</h2>
      <div className="space-y-4">
        {transfers.map((t) => (
          <div key={t.orderId} className="rounded-lg border border-edge bg-ink/60 p-4">
            <div className="mb-3 flex items-center justify-between">
              <div className="text-sm text-slate-200">
                {fmtAmount(BigInt(t.amount), REGISTRY[t.srcChainId].usdc.decimals, 4)} {t.token}{" "}
                <span className="text-slate-500">
                  {REGISTRY[t.srcChainId].shortName} → {REGISTRY[t.dstChainId].shortName}
                </span>
              </div>
              <span className="pill border-edge bg-panel text-slate-400">{t.mode}</span>
            </div>
            <OrderTimeline t={t} />
          </div>
        ))}
      </div>
    </div>
  );
}
