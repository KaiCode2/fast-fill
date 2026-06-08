"use client";

import { useAccount, useReadContract } from "wagmi";
import { erc20Abi } from "@/lib/abis/erc20";
import { REGISTRY, type SupportedChainId } from "@/lib/chains";
import { fmtAmount } from "@/lib/format";
import { bytes32ToAddress, deserializeOrder } from "@/lib/order";
import { useSelfFill } from "@/hooks/useSelfFill";
import type { TransferRecord } from "@/hooks/useTransfers";
import { InfoTip } from "./InfoTip";

const ACTIVE = new Set(["checking", "switching", "signing", "filling", "confirming"]);

/**
 * Lets the user optimistically fill their OWN in-flight order from the destination chain — the same
 * permissionless action a relayer performs. Shown only while an order is unfilled and only enabled
 * when the user holds enough of the output token on the destination. Self-service for the demo.
 */
export function SelfRelayButton({ t, onFilled }: { t: TransferRecord; onFilled?: () => void }) {
  const { address } = useAccount();
  const { selfFill, state } = useSelfFill();

  // Older transfers (initiated before this feature) didn't capture the order, so we can't rebuild the
  // exact struct `fill` needs — skip the affordance rather than offer something that would revert.
  const order = t.order ? deserializeOrder(t.order) : undefined;

  const dst = (order?.dstChainId ?? t.dstChainId) as SupportedChainId;
  const decimals = REGISTRY[dst].usdc.decimals;
  const need = order?.outputAmount ?? BigInt(t.outputAmount);
  const outputToken = order ? bytes32ToAddress(order.outputToken) : undefined;

  const { data: balance } = useReadContract({
    address: outputToken,
    abi: erc20Abi,
    functionName: "balanceOf",
    args: address ? [address] : undefined,
    chainId: dst,
    query: { enabled: !!address && !!outputToken },
  });

  if (!order) return null;

  const enough = typeof balance === "bigint" && balance >= need;
  const busy = ACTIVE.has(state.phase);

  async function onClick() {
    try {
      await selfFill(order!);
      onFilled?.();
    } catch {
      /* surfaced via state.error */
    }
  }

  return (
    <div className="mt-1 rounded-md border border-edge bg-panel/40 p-2">
      <div className="flex items-center justify-between gap-2">
        <span className="flex items-center gap-1 text-[11px] text-slate-400">
          Relay it yourself
          <InfoTip label="What is self-relay?">
            Fast-fill is permissionless — <span className="text-slate-300">anyone</span> holding {t.token} on{" "}
            {REGISTRY[dst].shortName} can fill your in-flight order and earn the premium. This button just lets
            you do it yourself; it&apos;s a convenience for the demo if no relayer steps in. You front{" "}
            {fmtAmount(need, decimals, 4)} {t.token} on {REGISTRY[dst].shortName} now (via an ERC-2612 permit, no
            approval tx) and are reimbursed when the bridge settles.
          </InfoTip>
        </span>
        <button
          className="btn-ghost shrink-0 px-2 py-1 text-[11px] disabled:opacity-50"
          disabled={busy || !enough || !address}
          onClick={onClick}
          title={!address ? "Connect a wallet" : !enough ? "Insufficient destination balance" : undefined}
        >
          {busy ? (state.message ?? "Working…") : "Fill it yourself"}
        </button>
      </div>
      {address && !enough && state.phase !== "done" && (
        <p className="mt-1 text-[11px] text-slate-500">
          Needs ≥ {fmtAmount(need, decimals, 4)} {t.token} on {REGISTRY[dst].shortName}
          {typeof balance === "bigint" ? ` — you have ${fmtAmount(balance, decimals, 4)}` : ""}.
        </p>
      )}
      {state.phase === "error" && state.error && <p className="mt-1 text-[11px] text-bad">{state.error}</p>}
      {state.phase === "done" && (
        <p className="mt-1 text-[11px] text-good">Filled — the recipient was paid. You&apos;re reimbursed at settlement.</p>
      )}
    </div>
  );
}
