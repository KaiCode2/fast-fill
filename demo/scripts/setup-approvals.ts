/**
 * One-time relayer setup: approve each destination adapter to pull the relayer's USDC / USD₮0 for
 * fills, and print a balance/allowance report. Idempotent — skips tokens already approved.
 *
 *   pnpm setup:approvals
 *
 * Reads demo/.env.local (RELAYER_PRIVATE_KEY, ALCHEMY_API_KEY, NEXT_PUBLIC adapter addresses).
 */
import { existsSync, readFileSync } from "node:fs";
import { dirname, resolve } from "node:path";
import { fileURLToPath } from "node:url";
import { createPublicClient, createWalletClient, formatUnits, http, maxUint256, type Address } from "viem";
import { privateKeyToAccount } from "viem/accounts";
import { erc20Abi } from "../src/lib/abis/erc20";
import { alchemyRpcUrl, REGISTRY, SUPPORTED_CHAIN_IDS, type SupportedChainId } from "../src/lib/chains";

const scriptDir = dirname(fileURLToPath(import.meta.url));

function loadEnvLocal() {
  const p = resolve(scriptDir, "..", ".env.local");
  if (!existsSync(p)) return;
  for (const raw of readFileSync(p, "utf8").split("\n")) {
    const line = raw.trim();
    if (!line || line.startsWith("#")) continue;
    const eq = line.indexOf("=");
    if (eq === -1) continue;
    const key = line.slice(0, eq).trim();
    let val = line.slice(eq + 1).trim();
    if ((val.startsWith('"') && val.endsWith('"')) || (val.startsWith("'") && val.endsWith("'"))) val = val.slice(1, -1);
    if (!(key in process.env)) process.env[key] = val;
  }
}
loadEnvLocal();

const PK = process.env.RELAYER_PRIVATE_KEY as `0x${string}` | undefined;
if (!PK) {
  console.error("RELAYER_PRIVATE_KEY is not set (in demo/.env.local). Aborting.");
  process.exit(1);
}
const account = privateKeyToAccount(PK);
const cctpAdapter = process.env.NEXT_PUBLIC_CCTP_ADAPTER as Address | undefined;
const oftAdapter = process.env.NEXT_PUBLIC_OFT_ADAPTER as Address | undefined;

function rpc(id: SupportedChainId): string {
  const key = process.env.ALCHEMY_API_KEY ?? process.env.NEXT_PUBLIC_ALCHEMY_API_KEY;
  return key ? alchemyRpcUrl(id, key) : REGISTRY[id].chain.rpcUrls.default.http[0];
}

const APPROVE_THRESHOLD = 10n ** 18n; // treat anything below this as "not approved"

async function run() {
  console.log(`Relayer: ${account.address}\n`);

  for (const id of SUPPORTED_CHAIN_IDS) {
    const meta = REGISTRY[id];
    const transport = http(rpc(id));
    const pub = createPublicClient({ chain: meta.chain, transport });
    const wallet = createWalletClient({ account, chain: meta.chain, transport });

    const native = await pub.getBalance({ address: account.address });
    console.log(`# ${meta.name} (${id}) — ${formatUnits(native, 18)} ETH`);

    const pairs: { token: Address; spender?: Address; symbol: string; decimals: number }[] = [
      { token: meta.usdc.address, spender: cctpAdapter, symbol: "USDC", decimals: meta.usdc.decimals },
    ];
    if (meta.usdt0) pairs.push({ token: meta.usdt0.address, spender: oftAdapter, symbol: "USDT0", decimals: meta.usdt0.decimals });

    for (const { token, spender, symbol, decimals } of pairs) {
      const bal = (await pub.readContract({ address: token, abi: erc20Abi, functionName: "balanceOf", args: [account.address] })) as bigint;
      if (!spender) {
        console.log(`  ${symbol}: ${formatUnits(bal, decimals)} (no adapter configured — skipping approval)`);
        continue;
      }
      const allowance = (await pub.readContract({ address: token, abi: erc20Abi, functionName: "allowance", args: [account.address, spender] })) as bigint;
      if (allowance >= APPROVE_THRESHOLD) {
        console.log(`  ${symbol}: ${formatUnits(bal, decimals)} — already approved`);
        continue;
      }
      console.log(`  ${symbol}: ${formatUnits(bal, decimals)} — approving ${spender}…`);
      const hash = await wallet.writeContract({ address: token, abi: erc20Abi, functionName: "approve", args: [spender, maxUint256] });
      await pub.waitForTransactionReceipt({ hash });
      console.log(`    approved (${hash})`);
    }
    console.log("");
  }
  console.log("Done. The relayer can now fill on every configured destination.");
}

run().catch((e) => {
  console.error(e);
  process.exit(1);
});
