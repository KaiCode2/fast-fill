import "server-only";
import type { Hex } from "viem";
import { cctpAdapterAbi, oftAdapterAbi } from "@/lib/abis/generated";
import type { SupportedChainId } from "@/lib/chains";
import { BRIDGE_CCTP } from "@/lib/order";
import { pollAttestation, settleCctp } from "./cctp";
import { pub, relayerAccount } from "./clients";
import { adapterAddressServer, SOURCE_CONFIRMATIONS } from "./env";
import { fillOrder } from "./fill";
import { lzScanStatus } from "./lz";
import { allJobs, getJob, indexPermit, patchJob, upsertJob, type OrderJob } from "./store";
import type { VerifiedOrder } from "./verify";

const ATTEST_TIMEOUT_MS = 30 * 60_000;
const TERMINAL = new Set(["SETTLED", "FAILED", "ATTEST_TIMEOUT"]);

// One ticker process-wide, anchored on globalThis for the same reason the store is (Next may load
// this module in more than one route bundle; we must not spin up a second interval per instance).
const tickerState = ((globalThis as unknown as { __fastFillTicker?: { started: boolean } }).__fastFillTicker ??= {
  started: false,
});
let ticking = false;

export function ensureTicker(): void {
  if (tickerState.started) return;
  tickerState.started = true;
  setInterval(() => void tick(), 4_000);
}

/** Create (or return the existing) job for a verified order and start the orchestrator. */
export function registerJob(
  v: VerifiedOrder,
  forwarding: boolean,
  opts?: { from?: string; permitNonce?: string },
): OrderJob {
  const existing = getJob(v.orderId);
  if (existing) return existing;
  const job: OrderJob = {
    orderId: v.orderId,
    order: v.order,
    bridgeType: v.bridgeType,
    srcChainId: v.order.srcChainId as SupportedChainId,
    dstChainId: v.order.dstChainId as SupportedChainId,
    srcTxHash: v.srcTxHash,
    srcBlockNumber: v.srcBlockNumber,
    forwarding,
    phase: "VERIFIED",
    attempts: 0,
    nextActionAt: 0,
    createdAt: Date.now(),
    updatedAt: Date.now(),
  };
  upsertJob(job);
  if (opts?.from && opts?.permitNonce) indexPermit(opts.from, opts.permitNonce, v.orderId);
  ensureTicker();
  void tick();
  return job;
}

const orderAbi = (bridgeType: number) => (bridgeType === BRIDGE_CCTP ? cctpAdapterAbi : oftAdapterAbi);

async function readOnchain(job: OrderJob): Promise<{ status: number; filler?: Hex } | undefined> {
  try {
    const rec = (await pub(job.dstChainId).readContract({
      address: adapterAddressServer(job.bridgeType),
      abi: orderAbi(job.bridgeType),
      functionName: "getOrder",
      args: [job.orderId],
    })) as { filler: Hex; status: number; fillTime: number };
    return { status: Number(rec.status), filler: rec.filler };
  } catch {
    return undefined;
  }
}

const defer = (job: OrderJob, ms: number) => patchJob(job.orderId, { nextActionAt: Date.now() + ms });
function backoff(job: OrderJob) {
  const n = Math.min(30_000, Math.round(5_000 * Math.pow(1.4, job.attempts)));
  patchJob(job.orderId, { attempts: job.attempts + 1, nextActionAt: Date.now() + n });
}

async function runFillStage(job: OrderJob) {
  // Reorg safety: wait a few source confirmations before fronting funds.
  if (job.srcBlockNumber !== undefined && SOURCE_CONFIRMATIONS > 0n) {
    const head = await pub(job.srcChainId).getBlockNumber();
    if (head - job.srcBlockNumber < SOURCE_CONFIRMATIONS) return defer(job, 4_000);
  }

  // Already filled/settled (by us on a prior tick, or by another relayer)?
  const oc = await readOnchain(job);
  if (oc && oc.status >= 1) {
    patchJob(job.orderId, { onchainStatus: oc.status, filler: oc.filler });
    patchJob(job.orderId, { phase: oc.status === 2 ? "SETTLED" : "ATTESTING" });
    return defer(job, 1_000);
  }

  patchJob(job.orderId, { phase: "FILLING" });
  const out = await fillOrder(job.order, job.bridgeType);
  const me = relayerAccount().address as Hex;
  if (out.kind === "filled") {
    patchJob(job.orderId, {
      fillTxHash: out.fillTxHash,
      payoutToRecipient: out.payout,
      feeToFiller: out.fee,
      onchainStatus: 1,
      filler: me,
    });
  } else if (out.kind === "already") {
    patchJob(job.orderId, { onchainStatus: 1 });
  } else if (out.kind === "insufficient") {
    patchJob(job.orderId, {
      error: `relayer liquidity short (have ${out.have}, need ${out.need}) — recipient gets funds at settlement`,
    });
  } else if (out.kind === "skipped") {
    patchJob(job.orderId, { error: out.reason });
  } else {
    patchJob(job.orderId, { error: out.error });
  }

  // Always proceed to the settlement watch — even unfilled, the bridge delivers to the recipient.
  patchJob(job.orderId, { phase: "ATTESTING" });
  defer(job, 1_000);
}

async function runSettleStage(job: OrderJob) {
  const oc = await readOnchain(job);
  if (oc?.status === 2) {
    patchJob(job.orderId, { phase: "SETTLED", onchainStatus: 2, filler: oc.filler, error: undefined });
    return;
  }
  if (oc?.status === 1 && !job.onchainStatus) patchJob(job.orderId, { onchainStatus: 1, filler: oc.filler });

  if (Date.now() - job.createdAt > ATTEST_TIMEOUT_MS) {
    patchJob(job.orderId, { phase: "ATTEST_TIMEOUT", error: "settlement not observed within 30 minutes" });
    return;
  }

  if (job.bridgeType !== BRIDGE_CCTP) {
    // OFT settlement is auto-delivered by the LayerZero executor; surface scan status + keep polling getOrder.
    const lz = await lzScanStatus(job.srcTxHash);
    if (lz) patchJob(job.orderId, { lzStatus: lz });
    return backoff(job);
  }

  const att = await pollAttestation(job.srcChainId, job.srcTxHash);
  patchJob(job.orderId, { attestationStatus: att.status });
  if (att.status !== "complete" || !att.message || !att.attestation) return backoff(job);

  if (!job.forwarding) {
    patchJob(job.orderId, {
      phase: "ATTESTED_AWAITING_USER",
      attestationMessage: att.message,
      attestation: att.attestation,
    });
    return;
  }

  // Relay Mint (executor-routed) orders bind the CCTP message's destinationCaller to the CctpExecutor,
  // so the adapter's direct `settle` can never consume them — the executor (primary relayer) does.
  // Once detected, stop re-simulating a settle that always reverts and just watch getOrder land it.
  if (job.executorRouted) {
    patchJob(job.orderId, { phase: "ATTESTING" });
    return backoff(job);
  }

  patchJob(job.orderId, { phase: "SETTLING" });
  try {
    const hash = await settleCctp(job.dstChainId, att.message, att.attestation);
    patchJob(job.orderId, { phase: "SETTLED", settleTxHash: hash, onchainStatus: 2, error: undefined });
  } catch (e) {
    const after = await readOnchain(job);
    if (after?.status === 2) {
      patchJob(job.orderId, { phase: "SETTLED", onchainStatus: 2, error: undefined });
    } else if (isExecutorRoutedRevert(e)) {
      // Expected for Relay Mint orders: the executor settles it. Defer and keep polling getOrder —
      // not a user-facing error.
      patchJob(job.orderId, { phase: "ATTESTING", executorRouted: true, error: undefined });
      backoff(job);
    } else {
      patchJob(job.orderId, { phase: "ATTESTING", error: clip(e) });
      backoff(job);
    }
  }
}

const clip = (e: unknown) => (e instanceof Error ? e.message : String(e)).slice(0, 160);

/** The CCTP MessageTransmitter revert when a non-bound caller tries to consume an executor-routed message. */
const isExecutorRoutedRevert = (e: unknown) =>
  /invalid caller for message/i.test(e instanceof Error ? e.message : String(e));

async function processJob(job: OrderJob) {
  if (job.inflight || Date.now() < job.nextActionAt) return;
  patchJob(job.orderId, { inflight: true });
  try {
    switch (job.phase) {
      case "RECEIVED":
      case "VERIFIED":
      case "FILLING":
        await runFillStage(job);
        break;
      case "ATTESTING":
      case "SETTLING":
        await runSettleStage(job);
        break;
      case "ATTESTED_AWAITING_USER": {
        const oc = await readOnchain(job);
        if (oc?.status === 2) patchJob(job.orderId, { phase: "SETTLED", onchainStatus: 2 });
        else defer(job, 15_000);
        break;
      }
      default:
        break;
    }
  } catch (e) {
    patchJob(job.orderId, { error: clip(e) });
    backoff(job);
  } finally {
    patchJob(job.orderId, { inflight: false });
  }
}

async function tick() {
  if (ticking) return;
  ticking = true;
  try {
    const open = allJobs().filter((j) => !TERMINAL.has(j.phase));
    await Promise.all(open.map((j) => processJob(j)));
  } finally {
    ticking = false;
  }
}

/** Manually nudge an order to settle now (used by the settle endpoint / forwarding toggle). */
export async function kickSettle(orderId: Hex): Promise<void> {
  const job = getJob(orderId);
  if (!job) return;
  patchJob(orderId, {
    forwarding: true,
    nextActionAt: 0,
    phase: job.phase === "ATTESTED_AWAITING_USER" ? "ATTESTING" : job.phase,
  });
  ensureTicker();
  void tick();
}
