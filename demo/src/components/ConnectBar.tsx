"use client";

import { ConnectButton } from "@rainbow-me/rainbowkit";
import Link from "next/link";

export function ConnectBar() {
  return (
    <header className="sticky top-0 z-20 border-b border-edge bg-ink/70 backdrop-blur">
      <div className="mx-auto flex max-w-5xl items-center justify-between gap-4 px-4 py-3">
        <div className="flex items-center gap-5">
          <Link href="/" className="flex items-center gap-2">
            <span className="grid h-8 w-8 place-items-center rounded-lg bg-accent font-bold text-ink">f</span>
            <div className="leading-tight">
              <div className="text-sm font-semibold text-slate-100">fast-fill</div>
              <div className="text-[11px] text-slate-500">optimistic cross-chain transfers</div>
            </div>
          </Link>
          <nav>
            <Link href="/docs" className="text-sm font-medium text-slate-400 transition hover:text-slate-200">
              Docs
            </Link>
          </nav>
        </div>
        <ConnectButton showBalance={false} accountStatus="address" chainStatus="icon" />
      </div>
    </header>
  );
}
