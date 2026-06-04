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
  settle: (orderId: Hex) => postJson<{ status: string }>(`/api/settle/${orderId}`, {}),
  status: async (orderId: Hex): Promise<OrderStatus> => {
    const res = await fetch(`/api/orders/${orderId}`);
    if (!res.ok) throw new Error(`status ${res.status}`);
    return (await res.json()) as OrderStatus;
  },
};
