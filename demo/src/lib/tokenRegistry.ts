import { getAddress, isAddress, type Address } from "viem";
import defaultTokenList from "@uniswap/default-token-list";
import { REGISTRY, SUPPORTED_CHAIN_IDS, isSupportedChain, type SupportedChainId } from "./chains";
import { erc20Abi } from "./abis/erc20";
import { publicClientFor } from "./viemClients";

/**
 * Token registry for swap-hook output tokens: resolve a user-entered symbol OR address to a token on a
 * given chain. Backed by the bundled Uniswap default token list (1458 tokens; filtered to the demo's
 * supported chains at module load). The app's own bridged stables (USDC / USD₮0) are seeded first so
 * they always resolve canonically.
 *
 * The Uniswap list has L2 gaps (e.g. Base has no USDT/WBTC by symbol) — for those, the user can paste a
 * contract address and `resolveTokenAsync` reads `symbol()/decimals()/name()` on-chain. So symbol entry
 * covers the common majors; address entry covers everything.
 */

export interface TokenMeta {
  chainId: SupportedChainId;
  address: Address; // checksummed
  symbol: string;
  name: string;
  decimals: number;
  logoURI?: string;
}

// chainId -> SYMBOL(upper) -> entries (array: a chain can list two tokens with the same symbol)
const bySymbol = new Map<SupportedChainId, Map<string, TokenMeta[]>>();
// chainId -> address(lower) -> entry
const byAddress = new Map<SupportedChainId, Map<string, TokenMeta>>();
// chainId -> all entries (for autocomplete)
const allByChain = new Map<SupportedChainId, TokenMeta[]>();

function indexToken(t: TokenMeta): void {
  const addrLower = t.address.toLowerCase();
  let addrMap = byAddress.get(t.chainId);
  if (!addrMap) byAddress.set(t.chainId, (addrMap = new Map()));
  if (addrMap.has(addrLower)) return; // first writer wins (seeds before the list)
  addrMap.set(addrLower, t);

  const key = t.symbol.toUpperCase();
  let symMap = bySymbol.get(t.chainId);
  if (!symMap) bySymbol.set(t.chainId, (symMap = new Map()));
  const arr = symMap.get(key);
  if (arr) arr.push(t);
  else symMap.set(key, [t]);

  const all = allByChain.get(t.chainId);
  if (all) all.push(t);
  else allByChain.set(t.chainId, [t]);
}

function toMeta(e: { chainId: number; address: string; symbol: string; name: string; decimals: number; logoURI?: string }): TokenMeta | null {
  if (!isSupportedChain(e.chainId)) return null;
  let address: Address;
  try {
    address = getAddress(e.address);
  } catch {
    return null; // malformed address in the list
  }
  return { chainId: e.chainId, address, symbol: e.symbol, name: e.name, decimals: e.decimals, logoURI: e.logoURI };
}

// Seed the app's own stables FIRST so "USDC"/"USDT" resolve to the exact tokens the bridge delivers.
for (const id of SUPPORTED_CHAIN_IDS) {
  const m = REGISTRY[id];
  indexToken({ chainId: id, address: getAddress(m.usdc.address), symbol: "USDC", name: "USD Coin", decimals: m.usdc.decimals });
  if (m.usdt0) {
    indexToken({ chainId: id, address: getAddress(m.usdt0.address), symbol: "USDT", name: "USD₮0", decimals: m.usdt0.decimals });
  }
}
// Then the broad Uniswap list.
for (const raw of defaultTokenList.tokens) {
  const meta = toMeta(raw);
  if (meta) indexToken(meta);
}
// Sort each chain's full list alphabetically by symbol so the picker shows a predictable order.
for (const list of allByChain.values()) {
  list.sort((a, b) => a.symbol.toLowerCase().localeCompare(b.symbol.toLowerCase()) || a.address.localeCompare(b.address));
}

/** Resolve a symbol (case-insensitive) or 0x address to a token on `chainId`. Sync (list/seed only). */
export function resolveToken(chainId: SupportedChainId, query: string): TokenMeta | undefined {
  const q = query.trim();
  if (!q) return undefined;
  if (isAddress(q)) return byAddress.get(chainId)?.get(q.toLowerCase());
  return bySymbol.get(chainId)?.get(q.toUpperCase())?.[0];
}

/** All tokens sharing a symbol on a chain (for disambiguating duplicates). */
export function resolveTokenCandidates(chainId: SupportedChainId, symbol: string): TokenMeta[] {
  return bySymbol.get(chainId)?.get(symbol.trim().toUpperCase()) ?? [];
}

/**
 * Prefix/substring search for the token picker. The candidate list is pre-sorted alphabetically, so
 * matches stay alphabetical (symbol-prefix matches rank above name/substring matches). `limit` is
 * optional — omit it to return every match (the picker is scrollable).
 */
export function searchTokens(chainId: SupportedChainId, query: string, limit?: number): TokenMeta[] {
  const all = allByChain.get(chainId) ?? [];
  const q = query.trim().toLowerCase();
  let out: TokenMeta[];
  if (!q) {
    out = all;
  } else {
    const starts: TokenMeta[] = [];
    const contains: TokenMeta[] = [];
    for (const t of all) {
      const sym = t.symbol.toLowerCase();
      if (sym.startsWith(q)) starts.push(t);
      else if (sym.includes(q) || t.name.toLowerCase().includes(q)) contains.push(t);
    }
    out = [...starts, ...contains];
  }
  return limit === undefined ? out : out.slice(0, limit);
}

/** Like `resolveToken` but falls back to reading ERC-20 metadata on-chain for an address not in the list. */
export async function resolveTokenAsync(chainId: SupportedChainId, query: string): Promise<TokenMeta | undefined> {
  const local = resolveToken(chainId, query);
  if (local) return local;
  const q = query.trim();
  if (!isAddress(q)) return undefined; // an unknown symbol can't be synthesized
  const address = getAddress(q);
  const client = publicClientFor(chainId);
  try {
    const [symbol, decimals, name] = await Promise.all([
      client.readContract({ address, abi: erc20Abi, functionName: "symbol" }) as Promise<string>,
      client.readContract({ address, abi: erc20Abi, functionName: "decimals" }) as Promise<number>,
      (client.readContract({ address, abi: erc20Abi, functionName: "name" }) as Promise<string>).catch(() => "Unknown Token"),
    ]);
    return { chainId, address, symbol, name, decimals: Number(decimals) };
  } catch {
    return undefined; // not an ERC-20 / no such contract on this chain
  }
}
