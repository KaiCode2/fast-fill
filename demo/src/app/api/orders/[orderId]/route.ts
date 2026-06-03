import "server-only";
import type { Hex } from "viem";
import { getJob, jobToStatus } from "@/lib/server/store";

export const runtime = "nodejs";
export const dynamic = "force-dynamic";

export async function GET(_req: Request, { params }: { params: Promise<{ orderId: string }> }) {
  const { orderId } = await params;
  const job = getJob(orderId as Hex);
  if (!job) return Response.json({ error: "unknown order" }, { status: 404 });
  return Response.json(jobToStatus(job));
}
