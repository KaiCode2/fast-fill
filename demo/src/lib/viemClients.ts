import { createPublicClient, http, type PublicClient } from "viem";
import { REGISTRY, type SupportedChainId } from "./chains";
import { rpcUrl } from "./config";

/**
 * Standalone viem public clients (per chain), independent of the connected wallet — used for reads,
 * receipt waits, and cross-chain quoting. Client-safe (NEXT_PUBLIC RPCs only).
 */
const cache: Partial<Record<SupportedChainId, PublicClient>> = {};

export function publicClientFor(chainId: SupportedChainId): PublicClient {
  if (!cache[chainId]) {
    cache[chainId] = createPublicClient({
      chain: REGISTRY[chainId].chain,
      transport: http(rpcUrl(chainId)),
    });
  }
  return cache[chainId]!;
}
