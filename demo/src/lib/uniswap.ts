import type { Address } from "viem";
import type { SupportedChainId } from "./chains";
import { publicClientFor } from "./viemClients";

/**
 * Crude Uniswap V3 quoting for the destination-side swap hook. The `UniswapSwapHook` swaps the
 * delivered stablecoin into `tokenOut` via `SwapRouter02.exactInputSingle` on a SINGLE fee tier, so
 * the front-end just has to pick the tier with the best quote and derive an `amountOutMinimum`.
 *
 * Quoting reads QuoterV2 on the DESTINATION chain — the swap runs there, on the post-fee amount that
 * actually arrives at the hook (not the pre-fee input). The quote is informational: the on-chain swap
 * is protected by `amountOutMinimum`, which is frozen into the signed order at sign time.
 */

// Uniswap V3 QuoterV2 — canonical deploys (verified live on-chain). OP/Arbitrum share an address.
export const QUOTER_V2: Record<SupportedChainId, Address> = {
  42161: "0x61fFE014bA17989E743c5F6cB21bF9697530B21e",
  10: "0x61fFE014bA17989E743c5F6cB21bF9697530B21e",
  8453: "0x3d4e44Eb1374240CE5F1B871ab261CD16335B76a",
};

/** Uniswap V3 fee tiers (bps * 100) to probe; reverts on missing pools are ignored. */
export const FEE_TIERS = [100, 500, 3000, 10000] as const;

/** Default swap slippage: 1.0%. amountOutMinimum is locked at sign time, so a touch of room avoids reverts. */
export const DEFAULT_SLIPPAGE_BPS = 100;

/**
 * QuoterV2's `quoteExactInputSingle` is declared non-`view` (it does a swap-and-revert internally) but
 * is fully callable via `eth_call`. We declare it `view` locally so viem's `readContract` accepts it.
 * Param ORDER trap: (tokenIn, tokenOut, amountIn, fee, sqrtPriceLimitX96) — `amountIn` before `fee`.
 */
export const quoterV2Abi = [
  {
    type: "function",
    name: "quoteExactInputSingle",
    stateMutability: "view",
    inputs: [
      {
        name: "params",
        type: "tuple",
        components: [
          { name: "tokenIn", type: "address" },
          { name: "tokenOut", type: "address" },
          { name: "amountIn", type: "uint256" },
          { name: "fee", type: "uint24" },
          { name: "sqrtPriceLimitX96", type: "uint160" },
        ],
      },
    ],
    outputs: [
      { name: "amountOut", type: "uint256" },
      { name: "sqrtPriceX96After", type: "uint160" },
      { name: "initializedTicksCrossed", type: "uint32" },
      { name: "gasEstimate", type: "uint256" },
    ],
  },
] as const;

export interface BestQuote {
  poolFee: number; // the winning fee tier
  amountOut: bigint; // expected output (tokenOut base units)
}

/**
 * Quote every fee tier in parallel and return the best (highest `amountOut`). Pools that don't exist
 * or are illiquid revert — those tiers are skipped. Returns null if no tier produces a quote (no pool).
 */
export async function quoteBestUniswapV3(
  dstChainId: SupportedChainId,
  tokenIn: Address,
  tokenOut: Address,
  amountIn: bigint,
): Promise<BestQuote | null> {
  if (amountIn <= 0n) return null;
  if (tokenIn.toLowerCase() === tokenOut.toLowerCase()) return null; // same token → no swap
  const client = publicClientFor(dstChainId);
  const quoter = QUOTER_V2[dstChainId];

  const results = await Promise.allSettled(
    FEE_TIERS.map(
      (fee) =>
        client.readContract({
          address: quoter,
          abi: quoterV2Abi,
          functionName: "quoteExactInputSingle",
          args: [{ tokenIn, tokenOut, amountIn, fee, sqrtPriceLimitX96: 0n }],
        }) as Promise<readonly [bigint, bigint, number, bigint]>,
    ),
  );

  let best: BestQuote | null = null;
  results.forEach((r, i) => {
    if (r.status !== "fulfilled") return; // pool missing / illiquid → reverted
    const amountOut = r.value[0];
    if (amountOut > 0n && (!best || amountOut > best.amountOut)) best = { poolFee: FEE_TIERS[i], amountOut };
  });
  return best;
}

/** amountOutMinimum = amountOut - slippage, integer math, in tokenOut base units. */
export function amountOutMinimum(amountOut: bigint, slippageBps = DEFAULT_SLIPPAGE_BPS): bigint {
  return (amountOut * BigInt(10_000 - slippageBps)) / 10_000n;
}
