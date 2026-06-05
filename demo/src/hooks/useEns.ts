"use client";

import { useQuery } from "@tanstack/react-query";
import type { Address } from "viem";
import { normalize } from "viem/ens";
import { ensClient } from "@/lib/ens";

/**
 * Reverse-resolve an address to its primary ENS name on mainnet. Returns null while loading, on
 * failure, or when the address has no primary name — callers fall back to the truncated address.
 */
export function useEnsName(address?: Address): string | null {
  const { data } = useQuery({
    queryKey: ["ensName", address],
    enabled: Boolean(address),
    staleTime: 10 * 60_000,
    gcTime: 30 * 60_000,
    retry: false,
    queryFn: async () => (address ? await ensClient.getEnsName({ address }) : null),
  });
  return data ?? null;
}

/** Resolve an ENS name's avatar URL (if any). Non-blocking; null while loading or when unset. */
export function useEnsAvatar(name: string | null): string | null {
  const { data } = useQuery({
    queryKey: ["ensAvatar", name],
    enabled: Boolean(name),
    staleTime: 10 * 60_000,
    gcTime: 30 * 60_000,
    retry: false,
    queryFn: async () => (name ? await ensClient.getEnsAvatar({ name: normalize(name) }) : null),
  });
  return data ?? null;
}
