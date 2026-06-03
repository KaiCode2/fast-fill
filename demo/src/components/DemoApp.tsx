"use client";

import { useAccount } from "wagmi";
import { useTransfers } from "@/hooks/useTransfers";
import { Balances } from "./Balances";
import { BridgeForm } from "./BridgeForm";
import { TransferHistory } from "./TransferHistory";

export function DemoApp() {
  const { isConnected } = useAccount();
  const { transfers, add } = useTransfers();

  if (!isConnected) {
    return (
      <div className="card text-center text-sm text-slate-400">
        Connect a wallet to see balances and start a transfer.
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
