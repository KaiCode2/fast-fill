"use client";

import { useQueryClient } from "@tanstack/react-query";
import type { OrderStatus } from "@/lib/api";
import { explorerTx, REGISTRY } from "@/lib/chains";
import { fmtAmount, shortHash } from "@/lib/format";
import { BRIDGE_CCTP } from "@/lib/order";
import { useOrderStatus } from "@/hooks/useOrderStatus";
import type { TransferRecord } from "@/hooks/useTransfers";
import { DocLink } from "./docs/DocLink";
import { SelfRelayButton } from "./SelfRelayButton";

type StepState = "done" | "active" | "pending" | "skipped";

function dot(s: StepState) {
  const cls =
    s === "done" ? "bg-good" : s === "active" ? "bg-accent animate-pulse" : s === "skipped" ? "bg-edge" : "bg-edge";
  return <span className={`mt-1 h-2.5 w-2.5 shrink-0 rounded-full ${cls}`} />;
}

export function OrderTimeline({ t }: { t: TransferRecord }) {
  const queryClient = useQueryClient();
  const { data, error } = useOrderStatus(t.orderId);
  const decimals = REGISTRY[t.srcChainId].usdc.decimals; // both tokens are 6dp
  const s: OrderStatus | undefined = data;

  const onchain = s?.onchainStatus ?? 0;
  const settled = onchain === 2 || s?.phase === "SETTLED";
  const filled = onchain >= 1 || Boolean(s?.fillTxHash);
  const pastFill = ["ATTESTING", "SETTLING", "ATTESTED_AWAITING_USER", "ATTEST_TIMEOUT"].includes(s?.phase ?? "");
  const noFill = !filled && pastFill; // relayer skipped the fill; bridge will deliver at settlement

  const fillState: StepState = filled ? "done" : s?.phase === "FILLING" ? "active" : noFill ? "skipped" : "pending";
  const settleState: StepState = settled ? "done" : filled || noFill || s ? "active" : "pending";

  return (
    <div className="space-y-3 text-sm">
      <div className="flex items-start gap-2">
        {dot("done")}
        <div>
          <div className="text-slate-200">Initiated on {REGISTRY[t.srcChainId].shortName}</div>
          {t.srcTxHash && (
            <a className="text-[11px] text-accent hover:underline" href={explorerTx(t.srcChainId, t.srcTxHash)} target="_blank" rel="noreferrer">
              {shortHash(t.srcTxHash)} ↗
            </a>
          )}
        </div>
      </div>

      <div className="flex items-start gap-2">
        {dot(fillState)}
        <div>
          <div className="text-slate-200">
            {filled
              ? "Filled — recipient paid instantly"
              : noFill
                ? "No relayer fill — recipient gets funds at settlement"
                : s?.phase === "FILLING"
                  ? "Filling…"
                  : "Awaiting fill"}
          </div>
          {filled && s?.payoutToRecipient && (
            <div className="text-[11px] text-slate-400">
              recipient +{fmtAmount(BigInt(s.payoutToRecipient), decimals, 4)} {t.token} · premium{" "}
              {s.feeToFiller ? fmtAmount(BigInt(s.feeToFiller), decimals, 4) : "—"}
            </div>
          )}
          {s?.fillTxHash && (
            <a className="text-[11px] text-accent hover:underline" href={explorerTx(t.dstChainId, s.fillTxHash)} target="_blank" rel="noreferrer">
              {shortHash(s.fillTxHash)} ↗
            </a>
          )}
        </div>
      </div>

      <div className="flex items-start gap-2">
        {dot(settleState)}
        <div>
          <div className="text-slate-200">
            {settled
              ? "Settled — bridge delivered"
              : t.bridgeType === BRIDGE_CCTP
                ? s?.attestationStatus === "complete"
                  ? "Attested — settling"
                  : "Awaiting Circle attestation"
                : "Awaiting LayerZero delivery"}
          </div>
          {settled && s?.arrivedAmount && (
            <div className="text-[11px] text-slate-400">
              arrived {fmtAmount(BigInt(s.arrivedAmount), decimals, 4)} {t.token}
              {s.surplusToRecipient && BigInt(s.surplusToRecipient) > 0n
                ? ` · surplus +${fmtAmount(BigInt(s.surplusToRecipient), decimals, 4)}`
                : ""}
            </div>
          )}
          {s?.settleTxHash && (
            <a className="text-[11px] text-accent hover:underline" href={explorerTx(t.dstChainId, s.settleTxHash)} target="_blank" rel="noreferrer">
              {shortHash(s.settleTxHash)} ↗
            </a>
          )}
          {s?.lzStatus && !settled && <div className="text-[11px] text-slate-500">LayerZero: {s.lzStatus}</div>}
        </div>
      </div>

      {!filled && !settled && (
        <SelfRelayButton
          t={t}
          onFilled={() => queryClient.invalidateQueries({ queryKey: ["orderStatus", t.orderId] })}
        />
      )}

      {error && <p className="text-[11px] text-slate-500">Relayer status unavailable (backend offline or not configured).</p>}
      {s?.error && <p className="text-[11px] text-warn">{s.error}</p>}
      <p className="text-[11px]">
        <DocLink href="/docs/architecture#3-order-lifecycle">Order lifecycle</DocLink>
      </p>
    </div>
  );
}
