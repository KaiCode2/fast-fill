import type { Address } from "viem";
import { alchemyRpcUrl, defaultRpcUrl, DEPLOYED, type SupportedChainId } from "./chains";

/**
 * Client-safe configuration, derived purely from NEXT_PUBLIC_* env vars (so it is identical on
 * the server and the client). Secrets (the relayer key, private RPCs) are read ONLY in the
 * server-only `clients.ts` module — never here.
 */

function pub(name: string): string | undefined {
  const v = process.env[name];
  return v && v.length > 0 ? v : undefined;
}

export const wcProjectId = pub("NEXT_PUBLIC_WC_PROJECT_ID") ?? "fast_fill_demo_placeholder";

/** Deployed adapter / registry addresses — default to the mainnet deployment, env overrides. */
export const adapters = {
  cctp: (pub("NEXT_PUBLIC_CCTP_ADAPTER") ?? DEPLOYED.cctpAdapter) as Address,
  oft: (pub("NEXT_PUBLIC_OFT_ADAPTER") ?? DEPLOYED.oftAdapter) as Address,
  config: (pub("NEXT_PUBLIC_FASTFILL_CONFIG") ?? DEPLOYED.fastFillConfig) as Address,
};

/** Addresses are baked in (deployed mainnet), so the app is always configured to bridge. */
export const contractsConfigured = true;

export function adapterAddress(bridgeType: number): Address {
  return bridgeType === 0 ? adapters.cctp : adapters.oft;
}

/** A single Alchemy key powers all chains; falls back to the chain's public RPC if unset. */
export const alchemyApiKey = pub("NEXT_PUBLIC_ALCHEMY_API_KEY");
export function rpcUrl(chainId: SupportedChainId): string {
  return alchemyApiKey ? alchemyRpcUrl(chainId, alchemyApiKey) : defaultRpcUrl(chainId);
}

/** Hard safety cap (USD, decimal) on a single transfer — this is a real-money demo. */
export const maxUsdPerTransfer = Number(pub("NEXT_PUBLIC_MAX_USD_PER_TRANSFER") ?? "2");
