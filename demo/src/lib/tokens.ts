import type { Address } from "viem";
import {
  REGISTRY,
  SUPPORTED_CHAIN_IDS,
  type SupportedChainId,
  type TokenInfo,
  type TokenSymbol,
} from "./chains";
import { BRIDGE_CCTP, BRIDGE_OFT } from "./order";

/**
 * Route gating. USDC (CCTP) bridges between any pair of {Arbitrum, Optimism, Base}. USD₮0 (OFT)
 * exists only on Arbitrum + Optimism, so it bridges between those two only.
 */

export const TOKEN_SYMBOLS: TokenSymbol[] = ["USDC", "USDT0"];

export function bridgeTypeForToken(symbol: TokenSymbol): number {
  return symbol === "USDC" ? BRIDGE_CCTP : BRIDGE_OFT;
}

/** Chains where this token is deployed. */
export function chainsForToken(symbol: TokenSymbol): SupportedChainId[] {
  return SUPPORTED_CHAIN_IDS.filter((id) =>
    symbol === "USDC" ? !!REGISTRY[id].usdc : !!REGISTRY[id].usdt0,
  );
}

export function tokensForChain(chainId: SupportedChainId): TokenInfo[] {
  const meta = REGISTRY[chainId];
  return meta.usdt0 ? [meta.usdc, meta.usdt0] : [meta.usdc];
}

export function isRouteSupported(
  symbol: TokenSymbol,
  src: SupportedChainId,
  dst: SupportedChainId,
): boolean {
  if (src === dst) return false;
  const chains = chainsForToken(symbol);
  return chains.includes(src) && chains.includes(dst);
}

/** Valid destinations for a given token + source chain. */
export function destinationsFor(symbol: TokenSymbol, src: SupportedChainId): SupportedChainId[] {
  return chainsForToken(symbol).filter((id) => id !== src);
}

export function getToken(chainId: SupportedChainId, symbol: TokenSymbol): TokenInfo {
  const t = symbol === "USDC" ? REGISTRY[chainId].usdc : REGISTRY[chainId].usdt0;
  if (!t) throw new Error(`${symbol} is not deployed on chain ${chainId}`);
  return t;
}

/** Identify a token symbol from its on-chain address (used to validate reconstructed orders). */
export function symbolByAddress(chainId: SupportedChainId, addr: Address): TokenSymbol | undefined {
  const m = REGISTRY[chainId];
  const a = addr.toLowerCase();
  if (m.usdc.address.toLowerCase() === a) return "USDC";
  if (m.usdt0 && m.usdt0.address.toLowerCase() === a) return "USDT0";
  return undefined;
}
