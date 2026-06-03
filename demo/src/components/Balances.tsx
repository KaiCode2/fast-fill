"use client";

import { REGISTRY } from "@/lib/chains";
import { fmtAmount } from "@/lib/format";
import { useBalances } from "@/hooks/useBalances";

export function Balances() {
  const { balances, isLoading, refetch } = useBalances();

  return (
    <div className="card">
      <div className="mb-3 flex items-center justify-between">
        <h2 className="text-sm font-semibold text-slate-100">Your balances</h2>
        <button onClick={() => refetch()} className="text-xs text-slate-400 hover:text-slate-200">
          {isLoading ? "…" : "refresh"}
        </button>
      </div>
      <div className="space-y-3">
        {balances.map((b) => (
          <div key={b.chainId} className="rounded-lg border border-edge bg-ink/60 p-3">
            <div className="mb-2 flex items-center justify-between">
              <span className="text-sm font-medium text-slate-200">{REGISTRY[b.chainId].shortName}</span>
              <span className="text-[11px] text-slate-500">
                {fmtAmount(b.native, 18, 4)} ETH
              </span>
            </div>
            <div className="flex flex-wrap gap-2">
              {b.tokens.map((t) => (
                <span key={t.symbol} className="pill border-edge bg-panel">
                  <span className="text-slate-400">{t.symbol}</span>
                  <span className="font-mono text-slate-100">{fmtAmount(t.amount, t.decimals, 2)}</span>
                </span>
              ))}
            </div>
          </div>
        ))}
      </div>
    </div>
  );
}
