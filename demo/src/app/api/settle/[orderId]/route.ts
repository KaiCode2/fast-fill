import "server-only";
import type { Hex } from "viem";
import { relayerConfigured } from "@/lib/server/env";
import { kickSettle } from "@/lib/server/relayer";
import { getJob, jobToStatus } from "@/lib/server/store";

export const runtime = "nodejs";
export const dynamic = "force-dynamic";

export async function POST(_req: Request, { params }: { params: Promise<{ orderId: string }> }) {
  if (!relayerConfigured) return Response.json({ error: "relayer not configured" }, { status: 503 });
  const { orderId } = await params;
  await kickSettle(orderId as Hex);
  const job = getJob(orderId as Hex);
  return job ? Response.json(jobToStatus(job)) : Response.json({ error: "unknown order" }, { status: 404 });
}
