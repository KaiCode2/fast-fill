import "server-only";
import type { Hex } from "viem";
import { LZ_SCAN_BASE } from "./env";

/** Best-effort LayerZero Scan status for a source tx (used to surface OFT delivery progress). */
export async function lzScanStatus(srcTxHash: Hex): Promise<string | undefined> {
  try {
    const res = await fetch(`${LZ_SCAN_BASE}/messages/tx/${srcTxHash}`, { cache: "no-store" });
    if (!res.ok) return undefined;
    const json = (await res.json()) as { data?: { status?: { name?: string } | string }[] };
    const status = json.data?.[0]?.status;
    return typeof status === "string" ? status : status?.name;
  } catch {
    return undefined;
  }
}
