import "server-only";
import type { Hex } from "viem";
import type { OrderPhase, OrderStatus } from "@/lib/api";
import type { SupportedChainId } from "@/lib/chains";
import type { Order } from "@/lib/order";

/** In-memory lifecycle store. Resets on redeploy — acceptable for a demo. */
export interface OrderJob {
  orderId: Hex;
  order: Order;
  bridgeType: number;
  srcChainId: SupportedChainId;
  dstChainId: SupportedChainId;
  srcTxHash: Hex;
  srcBlockNumber?: bigint;
  forwarding: boolean;
  phase: OrderPhase;
  fillTxHash?: Hex;
  settleTxHash?: Hex;
  payoutToRecipient?: bigint;
  feeToFiller?: bigint;
  arrivedAmount?: bigint;
  surplusToRecipient?: bigint;
  attestationStatus?: string;
  attestationMessage?: Hex;
  attestation?: Hex;
  lzStatus?: string;
  filler?: Hex;
  onchainStatus?: number;
  error?: string;
  attempts: number;
  nextActionAt: number;
  inflight?: boolean;
  createdAt: number;
  updatedAt: number;
}

const jobs = new Map<string, OrderJob>();
const byTx = new Map<string, Hex>();
const byPermit = new Map<string, Hex>();

const txKey = (chainId: number, txHash: string) => `${chainId}:${txHash.toLowerCase()}`;

export function getJob(orderId: Hex): OrderJob | undefined {
  return jobs.get(orderId.toLowerCase());
}

export function allJobs(): OrderJob[] {
  return [...jobs.values()];
}

export function jobByTx(chainId: number, txHash: string): OrderJob | undefined {
  const id = byTx.get(txKey(chainId, txHash));
  return id ? getJob(id) : undefined;
}

export function jobByPermit(from: string, nonce: string): OrderJob | undefined {
  const id = byPermit.get(`${from.toLowerCase()}:${nonce}`);
  return id ? getJob(id) : undefined;
}

export function upsertJob(job: OrderJob): OrderJob {
  job.updatedAt = Date.now();
  jobs.set(job.orderId.toLowerCase(), job);
  byTx.set(txKey(job.srcChainId, job.srcTxHash), job.orderId);
  return job;
}

export function indexPermit(from: string, nonce: string, orderId: Hex): void {
  byPermit.set(`${from.toLowerCase()}:${nonce}`, orderId);
}

export function patchJob(orderId: Hex, patch: Partial<OrderJob>): OrderJob | undefined {
  const job = getJob(orderId);
  if (!job) return undefined;
  Object.assign(job, patch, { updatedAt: Date.now() });
  return job;
}

/** Project a job to the client-facing status shape (bigint → string). */
export function jobToStatus(job: OrderJob): OrderStatus {
  return {
    orderId: job.orderId,
    bridgeType: job.bridgeType,
    srcChainId: job.srcChainId,
    dstChainId: job.dstChainId,
    phase: job.phase,
    onchainStatus: job.onchainStatus,
    filler: job.filler,
    srcTxHash: job.srcTxHash,
    fillTxHash: job.fillTxHash,
    settleTxHash: job.settleTxHash,
    payoutToRecipient: job.payoutToRecipient?.toString(),
    feeToFiller: job.feeToFiller?.toString(),
    arrivedAmount: job.arrivedAmount?.toString(),
    surplusToRecipient: job.surplusToRecipient?.toString(),
    attestationStatus: job.attestationStatus,
    lzStatus: job.lzStatus,
    error: job.error,
    updatedAt: job.updatedAt,
  };
}
