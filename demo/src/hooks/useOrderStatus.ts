"use client";

import { useQuery } from "@tanstack/react-query";
import type { Hex } from "viem";
import { api, type OrderStatus } from "@/lib/api";

const TERMINAL = new Set(["SETTLED", "FAILED", "ATTEST_TIMEOUT"]);

/** Polls the relayer for an order's lifecycle. Stops once terminal; tolerant of a missing backend. */
export function useOrderStatus(orderId: Hex, enabled = true) {
  return useQuery<OrderStatus>({
    queryKey: ["orderStatus", orderId],
    queryFn: () => api.status(orderId),
    enabled,
    retry: false,
    refetchInterval: (query) => {
      const phase = query.state.data?.phase;
      return phase && TERMINAL.has(phase) ? false : 5_000;
    },
  });
}
