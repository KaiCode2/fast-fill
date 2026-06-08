"use client";

import { useEffect, useState } from "react";
import { useAccount } from "wagmi";
import { useTransfers } from "@/hooks/useTransfers";
import { Balances } from "./Balances";
import { BridgeForm } from "./BridgeForm";
import { TransferHistory } from "./TransferHistory";

export function DemoApp() {
  const { address, status } = useAccount();
  const { transfers, add } = useTransfers();

  // With `ssr: true`, wagmi restores the previous session (cookie storage) on the client, so the
  // wallet is often already connected on load even though SSR rendered "disconnected". Decide what to
  // show only *after* mount — and treat the presence of an `address` as connected — so a restored
  // session shows the app immediately (matching the nav) instead of a dead "connect a wallet" screen
  // with no button. The first client render intentionally matches the server (no address) to avoid a
  // hydration mismatch; the mount effect then reconciles to the live wallet state.
  const [mounted, setMounted] = useState(false);
  useEffect(() => setMounted(true), []);

  if (!mounted || !address) {
    const connecting = mounted && (status === "connecting" || status === "reconnecting");
    return (
      <div className="card text-center text-sm text-slate-400">
        {connecting
          ? "Connecting your wallet…"
          : "Connect a wallet to see balances and start a transfer."}
      </div>
    );
  }

  return (
    <div className="grid gap-6 lg:grid-cols-5">
      <div className="space-y-6 lg:col-span-3">
        <BridgeForm onStarted={add} />
        <TransferHistory transfers={transfers} />
      </div>
      <div className="lg:col-span-2">
        <Balances />
      </div>
    </div>
  );
}
