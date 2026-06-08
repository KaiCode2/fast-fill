import { FINALITY_FAST, FINALITY_FINALIZED } from "./bridge";
import { isSupportedChain, type SupportedChainId, type TokenSymbol } from "./chains";
import { TOKEN_SYMBOLS } from "./tokens";

/**
 * Shareable bridge-form state, encoded into the URL hash fragment (e.g.
 * `#token=USDC&from=base&to=arbitrum&amount=1&speed=fast&mint=1`). The hash keeps the demo a static
 * client app (no server round-trip) while making any configured transfer a deep link. Decoding is
 * defensive — unknown/invalid keys are dropped so a hand-edited or stale URL never breaks the form;
 * BridgeForm reconciles the route against what each token actually supports.
 */
export interface FormUrlState {
  token: TokenSymbol;
  src: SupportedChainId;
  dst: SupportedChainId;
  amount: string;
  finality: number; // FINALITY_FAST | FINALITY_FINALIZED
  relayMint: boolean;
  action: "none" | "aave" | "uniswap";
  tokenOut?: string; // swap target (symbol or 0x address), uniswap action only
  slippageBps?: number; // uniswap action only
}

const CHAIN_SLUG: Record<SupportedChainId, string> = {
  42161: "arbitrum",
  10: "optimism",
  8453: "base",
};

// Accept a few friendly aliases so hand-typed links still resolve.
const SLUG_CHAIN: Record<string, SupportedChainId> = {
  arbitrum: 42161,
  arb: 42161,
  optimism: 10,
  op: 10,
  opt: 10,
  base: 8453,
};

const ACTIONS: FormUrlState["action"][] = ["none", "aave", "uniswap"];

/** Resolve a chain token (slug alias or numeric chainId) to a supported chainId, else undefined. */
function parseChain(value: string | null): SupportedChainId | undefined {
  if (!value) return undefined;
  const bySlug = SLUG_CHAIN[value.toLowerCase()];
  if (bySlug !== undefined) return bySlug;
  const byId = Number(value);
  return isSupportedChain(byId) ? byId : undefined;
}

/** Serialize form state to a hash fragment body (no leading `#`). */
export function encodeFormState(s: FormUrlState): string {
  const p = new URLSearchParams();
  p.set("token", s.token);
  p.set("from", CHAIN_SLUG[s.src]);
  p.set("to", CHAIN_SLUG[s.dst]);
  if (s.amount && s.amount !== "0") p.set("amount", s.amount);
  p.set("speed", s.finality === FINALITY_FINALIZED ? "finalized" : "fast");
  p.set("mint", s.relayMint ? "1" : "0");
  if (s.action !== "none") p.set("action", s.action);
  if (s.action === "uniswap") {
    if (s.tokenOut) p.set("tokenOut", s.tokenOut);
    if (s.slippageBps != null) p.set("slippage", String(s.slippageBps));
  }
  return p.toString();
}

/** Parse a hash fragment (with or without leading `#`) into a validated partial state. */
export function decodeFormState(hash: string): Partial<FormUrlState> {
  const raw = hash.startsWith("#") ? hash.slice(1) : hash;
  if (!raw) return {};
  const p = new URLSearchParams(raw);
  const out: Partial<FormUrlState> = {};

  const token = p.get("token");
  if (token && (TOKEN_SYMBOLS as string[]).includes(token)) out.token = token as TokenSymbol;

  const src = parseChain(p.get("from"));
  if (src !== undefined) out.src = src;

  const dst = parseChain(p.get("to"));
  if (dst !== undefined) out.dst = dst;

  const amount = p.get("amount");
  if (amount && /^\d*\.?\d*$/.test(amount)) out.amount = amount;

  const speed = p.get("speed");
  if (speed === "fast") out.finality = FINALITY_FAST;
  else if (speed === "finalized" || speed === "slow") out.finality = FINALITY_FINALIZED;

  const mint = p.get("mint");
  if (mint === "1" || mint === "true") out.relayMint = true;
  else if (mint === "0" || mint === "false") out.relayMint = false;

  const action = p.get("action");
  if (action && (ACTIONS as string[]).includes(action)) out.action = action as FormUrlState["action"];

  const tokenOut = p.get("tokenOut");
  if (tokenOut) out.tokenOut = tokenOut;

  const slippage = p.get("slippage");
  if (slippage && /^\d+$/.test(slippage)) out.slippageBps = Number(slippage);

  return out;
}
