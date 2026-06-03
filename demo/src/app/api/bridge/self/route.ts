import "server-only";
import type { NextRequest } from "next/server";
import type { Hex } from "viem";
import { relayerConfigured } from "@/lib/server/env";
import { rateLimit } from "@/lib/server/guards";
import { registerJob } from "@/lib/server/relayer";
import { jobByTx } from "@/lib/server/store";
import { reconstructAndVerify } from "@/lib/server/verify";

export const runtime = "nodejs";
export const dynamic = "force-dynamic";

export async function POST(req: NextRequest) {
  if (!relayerConfigured) {
    return Response.json({ error: "relayer not configured (set RELAYER_PRIVATE_KEY)" }, { status: 503 });
  }
  const ip = req.headers.get("x-forwarded-for")?.split(",")[0]?.trim() ?? "local";
  if (!rateLimit(`self:${ip}`)) return Response.json({ error: "rate limited" }, { status: 429 });

  let body: { txHash?: Hex; srcChainId?: number; forwarding?: boolean };
  try {
    body = await req.json();
  } catch {
    return Response.json({ error: "invalid json" }, { status: 400 });
  }
  const { txHash, srcChainId, forwarding } = body;
  if (!txHash || !srcChainId) {
    return Response.json({ error: "txHash and srcChainId are required" }, { status: 400 });
  }

  const existing = jobByTx(srcChainId, txHash);
  if (existing) return Response.json({ orderId: existing.orderId, status: "exists" });

  try {
    const verified = await reconstructAndVerify(Number(srcChainId), txHash);
    const job = registerJob(verified, Boolean(forwarding));
    return Response.json({ orderId: job.orderId, status: "verified_pending_fill" });
  } catch (e) {
    return Response.json({ error: e instanceof Error ? e.message : "verification failed" }, { status: 400 });
  }
}
