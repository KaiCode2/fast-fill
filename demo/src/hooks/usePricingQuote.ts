"use client";

import { useEffect, useMemo, useState } from "react";
import { api, type PricingQuoteRequest, type PricingQuoteResponse } from "@/lib/api";

export function usePricingQuote(req: PricingQuoteRequest | null): {
  quote: PricingQuoteResponse | null;
  loading: boolean;
  error: string | null;
} {
  const [quote, setQuote] = useState<PricingQuoteResponse | null>(null);
  const [loading, setLoading] = useState(false);
  const [error, setError] = useState<string | null>(null);

  const key = useMemo(() => (req ? JSON.stringify(req) : ""), [req]);

  useEffect(() => {
    if (!req || BigInt(req.amount) <= 0n) {
      setQuote(null);
      setLoading(false);
      setError(null);
      return;
    }

    let cancelled = false;
    setLoading(true);
    setError(null);
    const handle = setTimeout(() => {
      api
        .quotePricing(req)
        .then((q) => {
          if (!cancelled) setQuote(q);
        })
        .catch((e) => {
          if (!cancelled) {
            setQuote(null);
            setError(e instanceof Error ? e.message : "pricing quote failed");
          }
        })
        .finally(() => {
          if (!cancelled) setLoading(false);
        });
    }, 350);

    return () => {
      cancelled = true;
      clearTimeout(handle);
    };
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, [key]);

  return { quote, loading, error };
}
