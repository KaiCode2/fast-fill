import "server-only";
import type { NextRequest } from "next/server";
import type { PricingQuoteRequest } from "@/lib/api";
import { quoteBridgePricing } from "@/lib/server/pricing";
import { rateLimit } from "@/lib/server/guards";

export const runtime = "nodejs";
export const dynamic = "force-dynamic";

export async function POST(req: NextRequest) {
  const ip = req.headers.get("x-forwarded-for")?.split(",")[0]?.trim() ?? "local";
  if (!rateLimit(`pricing:${ip}`, 120, 60_000)) return Response.json({ error: "rate limited" }, { status: 429 });

  let body: PricingQuoteRequest;
  try {
    body = (await req.json()) as PricingQuoteRequest;
  } catch {
    return Response.json({ error: "invalid json" }, { status: 400 });
  }

  try {
    return Response.json(await quoteBridgePricing(body));
  } catch (e) {
    return Response.json({ error: e instanceof Error ? e.message : "pricing quote failed" }, { status: 400 });
  }
}
