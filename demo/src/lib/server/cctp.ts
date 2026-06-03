import "server-only";
import type { Hex } from "viem";
import { cctpAdapterAbi } from "@/lib/abis/generated";
import { REGISTRY, type SupportedChainId } from "@/lib/chains";
import { pub, relayerAccount, wallet, withChainLock } from "./clients";
import { adapterAddressServer, CIRCLE_IRIS_BASE } from "./env";

export interface AttestationResult {
  status: string; // "complete" | "pending_confirmations" | "pending" | ...
  message?: Hex;
  attestation?: Hex;
  forwardState?: string;
}

/** Poll Circle's CCTP v2 attestation service for a burn tx. */
export async function pollAttestation(srcChainId: SupportedChainId, burnTxHash: Hex): Promise<AttestationResult> {
  const domain = REGISTRY[srcChainId].cctpDomain;
  const url = `${CIRCLE_IRIS_BASE}/v2/messages/${domain}?transactionHash=${burnTxHash}`;
  const res = await fetch(url, { cache: "no-store" });
  if (res.status === 404) return { status: "pending" }; // not indexed yet
  if (!res.ok) return { status: `http_${res.status}` };
  const json = (await res.json()) as {
    messages?: { status?: string; message?: Hex; attestation?: Hex; forwardState?: string }[];
  };
  const m = json.messages?.[0];
  if (!m) return { status: "pending" };
  return { status: m.status ?? "pending", message: m.message, attestation: m.attestation, forwardState: m.forwardState };
}

/** Forward the attested message on the destination: wraps `receiveMessage` → mint → reimburse/disburse. */
export async function settleCctp(dstChainId: SupportedChainId, message: Hex, attestation: Hex): Promise<Hex> {
  const adapter = adapterAddressServer(0);
  return withChainLock(dstChainId, async () => {
    const { request } = await pub(dstChainId).simulateContract({
      account: relayerAccount(),
      address: adapter,
      abi: cctpAdapterAbi,
      functionName: "settle",
      args: [message, attestation],
    });
    const hash = await wallet(dstChainId).writeContract(request);
    await pub(dstChainId).waitForTransactionReceipt({ hash });
    return hash;
  });
}
