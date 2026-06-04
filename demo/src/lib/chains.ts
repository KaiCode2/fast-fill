import { arbitrum, base, optimism, type Chain } from "viem/chains";
import type { Address } from "viem";

/**
 * The canonical, public chain registry for the demo. Mirrors `script/config/Addresses.sol`
 * and `src/config/FastFillConfig.sol` (do NOT import Solidity — this is the off-chain mirror).
 *
 * Everything here is public on-chain data (token/bridge addresses, CCTP domains, LZ eids).
 * Deploy-specific values (our adapter addresses, RPC overrides, the relayer key) live in
 * `config.ts` (client-safe) and `clients.ts` (server-only).
 */

export const SUPPORTED_CHAIN_IDS = [42161, 10, 8453] as const;
export type SupportedChainId = (typeof SUPPORTED_CHAIN_IDS)[number];

// Deployed fast-fill contracts (mainnet). CREATE2-deterministic → same address on every chain.
// Env (NEXT_PUBLIC_*) overrides these if set. Source: repo DEPLOYMENTS.md.
export const DEPLOYED = {
  fastFillConfig: "0xaec766479DB174110958Bc45D141A2C5eF693DF5",
  cctpAdapter: "0x9FA37faBfA1Fd31Afe5A5F93e1c4Cd986b27bA75",
  oftAdapter: "0x45165aD55c5FADa4AEeD968469dB6e8e85b943Bf",
} as const;

// Same address on every chain (CREATE2 / canonical).
export const PERMIT2 = "0x000000000022D473030F116dDEE9F6B43aC78BA3" as const;
export const CCTP_TOKEN_MESSENGER_V2 = "0x28b5a0e9C621a5BadaA536219b3a228C8168cf5d" as const;
export const LZ_ENDPOINT_V2 = "0x1a44076050125825900e736c501f859c50fE728c" as const;

export type TokenSymbol = "USDC" | "USDT0";

export interface TokenInfo {
  symbol: TokenSymbol;
  address: Address;
  decimals: number; // USDC = 6, USD₮0 = 6 (verified on the OP fork, NOT 18)
}

export interface ChainMeta {
  chainId: SupportedChainId;
  chain: Chain;
  name: string;
  shortName: string;
  explorer: string; // base URL, no trailing slash
  cctpDomain: number;
  lzEid: number;
  usdc: TokenInfo;
  usdt0?: TokenInfo; // undefined on Base (no USD₮0 there)
  usdt0Oft?: Address; // OFT entrypoint we quote/send through (Arb/OP)
}

export const REGISTRY: Record<SupportedChainId, ChainMeta> = {
  42161: {
    chainId: 42161,
    chain: arbitrum,
    name: "Arbitrum One",
    shortName: "Arbitrum",
    explorer: "https://arbiscan.io",
    cctpDomain: 3,
    lzEid: 30_110,
    usdc: { symbol: "USDC", address: "0xaf88d065e77c8cC2239327C5EDb3A432268e5831", decimals: 6 },
    usdt0: { symbol: "USDT0", address: "0xFd086bC7CD5C481DCC9C85ebE478A1C0b69FCbb9", decimals: 6 },
    usdt0Oft: "0x14E4A1B13bf7F943c8ff7C51fb60FA964A298D92",
  },
  10: {
    chainId: 10,
    chain: optimism,
    name: "OP Mainnet",
    shortName: "Optimism",
    explorer: "https://optimistic.etherscan.io",
    cctpDomain: 2,
    lzEid: 30_111,
    usdc: { symbol: "USDC", address: "0x0b2C639c533813f4Aa9D7837CAf62653d097Ff85", decimals: 6 },
    usdt0: { symbol: "USDT0", address: "0x01bFF41798a0BcF287b996046Ca68b395DbC1071", decimals: 6 },
    usdt0Oft: "0xF03b4d9AC1D5d1E7c4cEf54C2A313b9fe051A0aD",
  },
  8453: {
    chainId: 8453,
    chain: base,
    name: "Base",
    shortName: "Base",
    explorer: "https://basescan.org",
    cctpDomain: 6,
    lzEid: 30_184,
    usdc: { symbol: "USDC", address: "0x833589fCD6eDb6E08f4c7C32D4f71b54bdA02913", decimals: 6 },
    // No USD₮0 on Base.
  },
};

export const CHAINS: ChainMeta[] = SUPPORTED_CHAIN_IDS.map((id) => REGISTRY[id]);
export const VIEM_CHAINS = [arbitrum, optimism, base] as const;

export function isSupportedChain(id: number): id is SupportedChainId {
  return (SUPPORTED_CHAIN_IDS as readonly number[]).includes(id);
}

export function chainMeta(id: number): ChainMeta {
  if (!isSupportedChain(id)) throw new Error(`Unsupported chainId ${id}`);
  return REGISTRY[id];
}

/** Look up a token's metadata by chain + symbol. Returns undefined if not deployed there. */
export function tokenInfo(chainId: SupportedChainId, symbol: TokenSymbol): TokenInfo | undefined {
  const meta = REGISTRY[chainId];
  return symbol === "USDC" ? meta.usdc : meta.usdt0;
}

export function explorerTx(chainId: SupportedChainId, hash: string): string {
  return `${REGISTRY[chainId].explorer}/tx/${hash}`;
}

export function explorerAddress(chainId: SupportedChainId, addr: string): string {
  return `${REGISTRY[chainId].explorer}/address/${addr}`;
}

/** Map a CCTP domain back to its chainId (used when reconstructing settlement). */
export function chainIdForCctpDomain(domain: number): SupportedChainId | undefined {
  return SUPPORTED_CHAIN_IDS.find((id) => REGISTRY[id].cctpDomain === domain);
}

/** Alchemy network slugs — one ALCHEMY_API_KEY builds an endpoint for every chain. */
const ALCHEMY_SLUG: Record<SupportedChainId, string> = {
  42161: "arb-mainnet",
  10: "opt-mainnet",
  8453: "base-mainnet",
};

export function alchemyRpcUrl(chainId: SupportedChainId, apiKey: string): string {
  return `https://${ALCHEMY_SLUG[chainId]}.g.alchemy.com/v2/${apiKey}`;
}

export function defaultRpcUrl(chainId: SupportedChainId): string {
  return REGISTRY[chainId].chain.rpcUrls.default.http[0];
}
