"use client";

import type { Address } from "viem";
import { useAccount, useBalance, useReadContracts } from "wagmi";
import { erc20Abi } from "@/lib/abis/erc20";
import { REGISTRY, SUPPORTED_CHAIN_IDS, type SupportedChainId, type TokenSymbol } from "@/lib/chains";
import { ZERO_ADDRESS } from "@/lib/format";

export interface TokenBalance {
  symbol: TokenSymbol;
  amount?: bigint;
  decimals: number;
}
export interface ChainBalance {
  chainId: SupportedChainId;
  native?: bigint;
  tokens: TokenBalance[];
}

type IndexEntry = { chainId: SupportedChainId; symbol: TokenSymbol; decimals: number };

/** Reads USDC + USD₮0 balances (per-chain multicall) and native gas balances for the account. */
export function useBalances() {
  const { address } = useAccount();
  const acct = (address ?? ZERO_ADDRESS) as Address;

  const contracts: {
    address: Address;
    abi: typeof erc20Abi;
    functionName: "balanceOf";
    args: readonly [Address];
    chainId: number;
  }[] = [];
  const index: IndexEntry[] = [];
  for (const id of SUPPORTED_CHAIN_IDS) {
    const m = REGISTRY[id];
    contracts.push({ address: m.usdc.address, abi: erc20Abi, functionName: "balanceOf", args: [acct], chainId: id });
    index.push({ chainId: id, symbol: "USDC", decimals: m.usdc.decimals });
    if (m.usdt0) {
      contracts.push({ address: m.usdt0.address, abi: erc20Abi, functionName: "balanceOf", args: [acct], chainId: id });
      index.push({ chainId: id, symbol: "USDT0", decimals: m.usdt0.decimals });
    }
  }

  const tokenReads = useReadContracts({
    contracts,
    allowFailure: true,
    query: { enabled: Boolean(address), refetchInterval: 15_000 },
  });

  // Native balances: one hook per chain (stable hook order).
  const nArb = useBalance({ address, chainId: 42161, query: { enabled: Boolean(address) } });
  const nOp = useBalance({ address, chainId: 10, query: { enabled: Boolean(address) } });
  const nBase = useBalance({ address, chainId: 8453, query: { enabled: Boolean(address) } });
  const native: Record<SupportedChainId, bigint | undefined> = {
    42161: nArb.data?.value,
    10: nOp.data?.value,
    8453: nBase.data?.value,
  };

  const byChain: Record<number, ChainBalance> = {};
  for (const id of SUPPORTED_CHAIN_IDS) byChain[id] = { chainId: id, native: native[id], tokens: [] };
  index.forEach((e, i) => {
    const r = tokenReads.data?.[i];
    const amount = r && r.status === "success" ? (r.result as bigint) : undefined;
    byChain[e.chainId].tokens.push({ symbol: e.symbol, amount, decimals: e.decimals });
  });

  return {
    balances: SUPPORTED_CHAIN_IDS.map((id) => byChain[id]),
    isLoading: tokenReads.isLoading,
    // Awaitable so callers can show a spinner that lasts the whole refresh (token multicall + natives).
    refetch: () => Promise.all([tokenReads.refetch(), nArb.refetch(), nOp.refetch(), nBase.refetch()]),
  };
}
