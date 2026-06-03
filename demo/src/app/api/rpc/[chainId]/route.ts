import "server-only";
import type { NextRequest } from "next/server";
import { isSupportedChain } from "@/lib/chains";
import { serverRpcUrl } from "@/lib/server/env";
import { rateLimit } from "@/lib/server/guards";

export const runtime = "nodejs";
export const dynamic = "force-dynamic";

/**
 * Read-only JSON-RPC proxy. The browser points its viem / wagmi transports at `/api/rpc/[chainId]`
 * (see `lib/config.ts`); we forward to Alchemy server-side so the API key never reaches the client.
 * Calls are rate-limited per IP and restricted to read methods — the connected wallet (not this
 * transport) is what signs and broadcasts transactions, so there is no reason to proxy writes.
 */
const ALLOWED_METHODS = new Set([
  "eth_chainId",
  "eth_blockNumber",
  "eth_call",
  "eth_getBalance",
  "eth_getCode",
  "eth_getStorageAt",
  "eth_getTransactionCount",
  "eth_getTransactionReceipt",
  "eth_getTransactionByHash",
  "eth_getBlockByNumber",
  "eth_getBlockByHash",
  "eth_getBlockReceipts",
  "eth_getLogs",
  "eth_estimateGas",
  "eth_gasPrice",
  "eth_feeHistory",
  "eth_maxPriorityFeePerGas",
  "net_version",
  "web3_clientVersion",
]);

/** A request (or every entry of a JSON-RPC batch) must target an allow-listed read method. */
function methodsAllowed(body: unknown): boolean {
  const calls = Array.isArray(body) ? body : [body];
  if (calls.length === 0) return false;
  return calls.every(
    (c) => typeof c === "object" && c !== null && ALLOWED_METHODS.has((c as { method?: string }).method ?? ""),
  );
}

export async function POST(req: NextRequest, { params }: { params: Promise<{ chainId: string }> }) {
  const { chainId: chainIdParam } = await params;
  const chainId = Number(chainIdParam);
  if (!isSupportedChain(chainId)) {
    return Response.json({ error: `unsupported chainId ${chainIdParam}` }, { status: 400 });
  }

  const ip = req.headers.get("x-forwarded-for")?.split(",")[0]?.trim() ?? "local";
  // RPC is chatty (balance multicalls, receipt polling) — a higher bucket than the relayer routes.
  if (!rateLimit(`rpc:${ip}`, 600, 60_000)) {
    return Response.json({ error: "rate limited" }, { status: 429 });
  }

  let body: unknown;
  try {
    body = await req.json();
  } catch {
    return Response.json({ error: "invalid json" }, { status: 400 });
  }
  if (!methodsAllowed(body)) {
    return Response.json({ error: "method not allowed" }, { status: 403 });
  }

  try {
    const upstream = await fetch(serverRpcUrl(chainId), {
      method: "POST",
      headers: { "content-type": "application/json" },
      body: JSON.stringify(body),
    });
    const text = await upstream.text();
    return new Response(text, {
      status: upstream.status,
      headers: { "content-type": upstream.headers.get("content-type") ?? "application/json" },
    });
  } catch (e) {
    return Response.json({ error: e instanceof Error ? e.message : "rpc proxy failed" }, { status: 502 });
  }
}
