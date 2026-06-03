import "server-only";
import { erc20Abi } from "@/lib/abis/erc20";
import { REGISTRY, SUPPORTED_CHAIN_IDS } from "@/lib/chains";
import { pub, relayerAccount } from "@/lib/server/clients";
import { adapterAddressServer, relayerConfigured } from "@/lib/server/env";

export const runtime = "nodejs";
export const dynamic = "force-dynamic";

async function allowance(p: ReturnType<typeof pub>, token: `0x${string}`, owner: `0x${string}`, spender: `0x${string}`) {
  try {
    return (
      (await p.readContract({ address: token, abi: erc20Abi, functionName: "allowance", args: [owner, spender] })) as bigint
    ).toString();
  } catch {
    return undefined;
  }
}

export async function GET() {
  if (!relayerConfigured) return Response.json({ configured: false });
  try {
    const relayer = relayerAccount().address;
    const chains = await Promise.all(
      SUPPORTED_CHAIN_IDS.map(async (id) => {
        const meta = REGISTRY[id];
        const p = pub(id);
        const native = await p.getBalance({ address: relayer });
        const usdc = (await p.readContract({
          address: meta.usdc.address,
          abi: erc20Abi,
          functionName: "balanceOf",
          args: [relayer],
        })) as bigint;
        const usdt0 = meta.usdt0
          ? ((await p.readContract({
              address: meta.usdt0.address,
              abi: erc20Abi,
              functionName: "balanceOf",
              args: [relayer],
            })) as bigint)
          : undefined;
        return {
          chainId: id,
          name: meta.shortName,
          native: native.toString(),
          usdc: usdc.toString(),
          usdt0: usdt0?.toString(),
          cctpAllowance: await allowance(p, meta.usdc.address, relayer, safeAdapter(0)),
          oftAllowance: meta.usdt0 ? await allowance(p, meta.usdt0.address, relayer, safeAdapter(1)) : undefined,
        };
      }),
    );
    return Response.json({ configured: true, relayer, chains });
  } catch (e) {
    return Response.json({ configured: true, error: e instanceof Error ? e.message : "health failed" }, { status: 500 });
  }
}

function safeAdapter(bridgeType: number): `0x${string}` {
  try {
    return adapterAddressServer(bridgeType);
  } catch {
    return "0x0000000000000000000000000000000000000000";
  }
}
