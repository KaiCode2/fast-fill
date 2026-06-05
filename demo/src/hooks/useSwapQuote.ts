"use client";

import { useEffect, useState } from "react";
import { useQuery } from "@tanstack/react-query";
import type { Address } from "viem";
import type { SupportedChainId } from "@/lib/chains";
import { amountOutMinimum, DEFAULT_SLIPPAGE_BPS, quoteBestUniswapV3 } from "@/lib/uniswap";

export interface SwapQuote {
  poolFee: number; // winning Uniswap V3 fee tier
  amountOut: bigint; // expected output (tokenOut base units)
  amountOutMin: bigint; // amountOut − slippage, frozen into the order at sign time
}

interface UseSwapQuoteArgs {
  dstChainId: SupportedChainId;
  tokenIn?: Address; // the bridged stable on the destination chain
  tokenOut?: Address; // resolved swap-output token
  amountIn: bigint; // the post-fee amount that arrives at the hook
  slippageBps?: number;
  enabled?: boolean;
}

/** Debounce a value so a fast-changing input settles before it drives a network query. */
function useDebounced<T>(value: T, ms = 350): T {
  const [v, setV] = useState(value);
  useEffect(() => {
    const h = setTimeout(() => setV(value), ms);
    return () => clearTimeout(h);
  }, [value, ms]);
  return v;
}

/**
 * Live "best pool" Uniswap V3 quote for the destination swap hook. Debounced + cached via react-query.
 * `noPool` means we resolved the token and queried, but no fee tier has a pool (distinct from a transient
 * `error`). The authoritative slippage protection is `amountOutMin`, baked into the signed order.
 */
export function useSwapQuote(args: UseSwapQuoteArgs): {
  quote: SwapQuote | null;
  loading: boolean;
  error: boolean;
  noPool: boolean;
} {
  const slippageBps = args.slippageBps ?? DEFAULT_SLIPPAGE_BPS;
  const debouncedAmount = useDebounced(args.amountIn.toString());
  const sameToken = Boolean(
    args.tokenIn && args.tokenOut && args.tokenIn.toLowerCase() === args.tokenOut.toLowerCase(),
  );
  const active = Boolean(
    args.enabled !== false && args.tokenIn && args.tokenOut && !sameToken && BigInt(debouncedAmount) > 0n,
  );

  const q = useQuery({
    queryKey: ["uniV3Quote", args.dstChainId, args.tokenIn, args.tokenOut, debouncedAmount, slippageBps],
    enabled: active,
    staleTime: 12_000, // quotes go stale fast
    retry: false, // a revert is "no pool", not a transient error to retry
    queryFn: async (): Promise<SwapQuote | null> => {
      const best = await quoteBestUniswapV3(args.dstChainId, args.tokenIn!, args.tokenOut!, BigInt(debouncedAmount));
      if (!best) return null;
      return { ...best, amountOutMin: amountOutMinimum(best.amountOut, slippageBps) };
    },
  });

  return {
    quote: (q.data ?? null) as SwapQuote | null,
    loading: active && (q.isLoading || q.isFetching),
    error: q.isError,
    noPool: active && q.isSuccess && q.data === null,
  };
}
