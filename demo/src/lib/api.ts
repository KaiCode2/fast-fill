import type { Hex } from "viem";
import type { SupportedChainId, TokenSymbol } from "./chains";

/** The three ways a user can start a transfer. */
export type SubmitMode = "standard" | "permit2612" | "permit2";

export type SerializedOrder = Record<string, string | number>;

/** Backend lifecycle phases (mirrors `relayer.ts`). */
export type OrderPhase =
  | "RECEIVED"
  | "VERIFIED"
  | "FILLING"
  | "FILLED"
  | "FILLED_BY_OTHER"
  | "ATTESTING"
  | "SETTLING"
  | "SETTLED"
  | "ATTESTED_AWAITING_USER"
  | "FAILED_LIQUIDITY"
  | "ATTEST_TIMEOUT"
  | "FAILED";

export interface OrderStatus {
  orderId: Hex;
  bridgeType: number;
  srcChainId: SupportedChainId;
  dstChainId: SupportedChainId;
  phase: OrderPhase;
  onchainStatus?: number; // 0 None, 1 Filled, 2 Settled (from getOrder)
  filler?: Hex;
  srcTxHash?: Hex;
  fillTxHash?: Hex;
  settleTxHash?: Hex;
  payoutToRecipient?: string;
  feeToFiller?: string;
  arrivedAmount?: string;
  surplusToRecipient?: string;
  attestationStatus?: string;
  lzStatus?: string;
  error?: string;
  updatedAt?: number;
}

export interface SelfBridgeRequest {
  txHash: Hex;
  srcChainId: SupportedChainId;
  forwarding: boolean;
  // Optional cross-check hints (the backend re-derives and verifies regardless).
  order?: SerializedOrder;
}

export interface GaslessBridgeRequest {
  bridgeType: number;
  token: TokenSymbol;
  srcChainId: SupportedChainId;
  dstChainId: SupportedChainId;
  from: Hex;
  recipient: Hex; // bytes32
  inputAmount: string;
  outputAmount: string;
  deliveryWindow: number;
  discountRate: string;
  baseFee: string;
  forwarding: boolean;
  // Destination execution (empty/0 = deliver funds only):
  hookData: Hex;
  callbackGasLimit: string;
  // CCTP-only:
  maxFee?: string;
  mintFee?: string;
  minFinalityThreshold?: number;
  // OFT-only (the relayer rebuilds extraOptions from these so keccak256 matches the witness):
  lzReceiveGas?: string;
  composeGas?: string;
  permit: { nonce: string; deadline: string; signature: Hex };
}

export interface PricingQuoteRequest {
  bridgeType: number;
  token: TokenSymbol;
  srcChainId: SupportedChainId;
  dstChainId: SupportedChainId;
  amount: string;
  finality: number;
  relayMint: boolean;
  deliveryWindow: string;
  callbackGasLimit: string;
}

export interface PricingQuoteResponse {
  params: {
    maxFee: string;
    mintFee: string;
    baseFee: string;
    discountRate: string;
    deliveryWindow: string;
    outputAmount: string;
  };
  breakdown: {
    gasPriceWei: string;
    ethUsd: string;
    cctpMinimumFeeBps?: string;
    cctpProtocolFee?: string;
    cctpMaxFeeBuffer?: string;
    fillGasUnits: string;
    fillGasFee: string;
    directSettleGasUnits: string;
    directSettleGasFee: string;
    executorGasUnits: string;
    executorGasFee: string;
    secondsSaved: string;
    targetTimeFee: string;
  };
  // Single apples-to-apples Circle reference matching the request's settlement speed + forwarding.
  comparison?: {
    speed: "fast" | "slow"; // mirrors the selected CCTP finality
    forwarding: boolean; // whether the Circle reference includes the forwarding (mint) fee
    circleDirect: string; // total Circle cost for this exact transfer's settings
    circleProtocolFee: string;
    circleForwardFee: string; // 0 when forwarding is off
    circleClaimGas: string; // gas to self-mint the USDC; 0 when forwarding is on
    circleExecGas: string; // gas for a separate destination action tx (deposit/swap); 0 when no action
    cctpDirectReceived: string; // amount − circleDirect: what the recipient nets via CCTP directly
    fastFillEstimated: string; // fast-fill total cost (fees) for the same settings
    savings: string; // signed circleDirect − fastFillEstimated
  };
}

async function postJson<T>(url: string, body: unknown): Promise<T> {
  const res = await fetch(url, {
    method: "POST",
    headers: { "content-type": "application/json" },
    body: JSON.stringify(body),
  });
  const json = await res.json().catch(() => ({}));
  if (!res.ok) throw new Error(json?.error || `${url} failed (${res.status})`);
  return json as T;
}

export const api = {
  submitSelf: (req: SelfBridgeRequest) =>
    postJson<{ orderId: Hex; status: string }>("/api/bridge/self", req),
  submitGasless: (req: GaslessBridgeRequest) =>
    postJson<{ orderId: Hex; srcTxHash: Hex; status: string }>("/api/bridge/gasless", req),
  quotePricing: (req: PricingQuoteRequest) => postJson<PricingQuoteResponse>("/api/bridge/pricing", req),
  settle: (orderId: Hex) => postJson<{ status: string }>(`/api/settle/${orderId}`, {}),
  status: async (orderId: Hex): Promise<OrderStatus> => {
    const res = await fetch(`/api/orders/${orderId}`);
    if (!res.ok) throw new Error(`status ${res.status}`);
    return (await res.json()) as OrderStatus;
  },
};

const sleep = (ms: number) => new Promise<void>((r) => setTimeout(r, ms));

// The Next.js relayer is a FALLBACK — the Rust relayer watches OrderCreated on-chain and is primary.
// We still try hard to hand off the self-submitted burn: (1) a short initial delay lets the backend's
// RPC index the just-mined tx before it verifies + fills, and (2) we retry on failure so a transient
// blip (cold start, RPC lag, rate-limit) doesn't silently drop the fallback notification.
const SELF_HANDOFF_INITIAL_DELAY_MS = 100;
const SELF_HANDOFF_RETRY_MS = [300, 800, 1500];

export type SelfHandoffResult =
  | { ok: true; orderId: Hex; status: string }
  | { ok: false; attempts: number; error: string };

export async function submitSelfWithRetry(
  req: SelfBridgeRequest,
  onAttempt?: (attempt: number, total: number) => void,
): Promise<SelfHandoffResult> {
  const total = SELF_HANDOFF_RETRY_MS.length + 1;
  await sleep(SELF_HANDOFF_INITIAL_DELAY_MS);
  let lastErr = "unknown error";
  for (let attempt = 1; attempt <= total; attempt++) {
    onAttempt?.(attempt, total);
    try {
      const res = await api.submitSelf(req);
      return { ok: true, ...res };
    } catch (e) {
      lastErr = e instanceof Error ? e.message : String(e);
      if (attempt < total) await sleep(SELF_HANDOFF_RETRY_MS[attempt - 1]);
    }
  }
  return { ok: false, attempts: total, error: lastErr };
}
