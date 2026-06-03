import "server-only";
import type { NextRequest } from "next/server";
import type { GaslessBridgeRequest } from "@/lib/api";
import { relayerConfigured } from "@/lib/server/env";
import { submitGasless } from "@/lib/server/gasless";
import { rateLimit } from "@/lib/server/guards";

export const runtime = "nodejs";
export const dynamic = "force-dynamic";

export async function POST(req: NextRequest) {
  if (!relayerConfigured) {
    return Response.json({ error: "relayer not configured (set RELAYER_PRIVATE_KEY)" }, { status: 503 });
  }
  const ip = req.headers.get("x-forwarded-for")?.split(",")[0]?.trim() ?? "local";
  if (!rateLimit(`gasless:${ip}`)) return Response.json({ error: "rate limited" }, { status: 429 });

  let body: GaslessBridgeRequest;
  try {
    body = (await req.json()) as GaslessBridgeRequest;
  } catch {
    return Response.json({ error: "invalid json" }, { status: 400 });
  }

  try {
    const { orderId, srcTxHash } = await submitGasless(body);
    return Response.json({ orderId, srcTxHash, status: "initiated" });
  } catch (e) {
    return Response.json({ error: e instanceof Error ? e.message : "gasless submit failed" }, { status: 400 });
  }
}
