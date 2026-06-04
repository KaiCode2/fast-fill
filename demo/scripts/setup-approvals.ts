/**
 * One-time relayer setup: approve each destination adapter to pull the relayer's supported tokens for
 * fills, and print a balance/allowance report. Idempotent — skips tokens already approved.
 *
 *   pnpm setup:approvals
 *
 * Reads demo/.env.local (RELAYER_PRIVATE_KEY, ALCHEMY_API_KEY). Adapter addresses are the latest
 * deterministic mainnet deployments documented in DEPLOYMENTS.md.
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

// Source: DEPLOYMENTS.md. Executor-enabled CCTP adapter (replaces the pre-executor 0xccee… historical).
// CREATE2-deterministic — identical on every deployed chain.
const CCTP_ADAPTER = "0x9FA37faBfA1Fd31Afe5A5F93e1c4Cd986b27bA75" as const satisfies Address;
const OFT_ADAPTERS = {
  USDT0: "0x45165aD55c5FADa4AEeD968469dB6e8e85b943Bf",
  USDe: "0x27989367741A6662daeFd5CabeC4f40323461778",
  sUSDe: "0x58E5315Ab8B975737d2655e838De12a2c48b497D",
  ENA: "0x3291098E6F0e7C206DfB64c54E6D4927e42262b8",
} as const satisfies Record<string, Address>;

type Approval = {
  token: Address;
  spender: Address;
  symbol: string;
  decimals: number;
  bridge: "CCTP" | "OFT";
};

const OFT_TOKENS: Partial<Record<SupportedChainId, Approval[]>> = {
  42161: [
    { token: "0xFd086bC7CD5C481DCC9C85ebE478A1C0b69FCbb9", spender: OFT_ADAPTERS.USDT0, symbol: "USDT0", decimals: 6, bridge: "OFT" },
    { token: "0x5d3a1Ff2b6BAb83b63cd9AD0787074081a52ef34", spender: OFT_ADAPTERS.USDe, symbol: "USDe", decimals: 18, bridge: "OFT" },
    { token: "0x211Cc4DD073734dA055fbF44a2b4667d5E5fE5d2", spender: OFT_ADAPTERS.sUSDe, symbol: "sUSDe", decimals: 18, bridge: "OFT" },
    { token: "0x58538e6A46E07434d7E7375Bc268D3cb839C0133", spender: OFT_ADAPTERS.ENA, symbol: "ENA", decimals: 18, bridge: "OFT" },
  ],
  10: [
    { token: "0x01bFF41798a0BcF287b996046Ca68b395DbC1071", spender: OFT_ADAPTERS.USDT0, symbol: "USDT0", decimals: 6, bridge: "OFT" },
    { token: "0x5d3a1Ff2b6BAb83b63cd9AD0787074081a52ef34", spender: OFT_ADAPTERS.USDe, symbol: "USDe", decimals: 18, bridge: "OFT" },
    { token: "0x211Cc4DD073734dA055fbF44a2b4667d5E5fE5d2", spender: OFT_ADAPTERS.sUSDe, symbol: "sUSDe", decimals: 18, bridge: "OFT" },
    { token: "0x58538e6A46E07434d7E7375Bc268D3cb839C0133", spender: OFT_ADAPTERS.ENA, symbol: "ENA", decimals: 18, bridge: "OFT" },
  ],
  8453: [
    { token: "0x5d3a1Ff2b6BAb83b63cd9AD0787074081a52ef34", spender: OFT_ADAPTERS.USDe, symbol: "USDe", decimals: 18, bridge: "OFT" },
    { token: "0x211Cc4DD073734dA055fbF44a2b4667d5E5fE5d2", spender: OFT_ADAPTERS.sUSDe, symbol: "sUSDe", decimals: 18, bridge: "OFT" },
    { token: "0x58538e6A46E07434d7E7375Bc268D3cb839C0133", spender: OFT_ADAPTERS.ENA, symbol: "ENA", decimals: 18, bridge: "OFT" },
  ],
};

function rpc(id: SupportedChainId): string {
  const key = process.env.ALCHEMY_API_KEY;
  return key ? alchemyRpcUrl(id, key) : REGISTRY[id].chain.rpcUrls.default.http[0];
}

const APPROVE_THRESHOLD = 10n ** 18n; // treat anything below this as "not approved"

async function processChain(id: SupportedChainId): Promise<string[]> {
  const meta = REGISTRY[id];
  const transport = http(rpc(id));
  const pub = createPublicClient({ chain: meta.chain, transport });
  const wallet = createWalletClient({ account, chain: meta.chain, transport });
  const lines: string[] = [];

  const native = await pub.getBalance({ address: account.address });
  lines.push(`# ${meta.name} (${id}) — ${formatUnits(native, 18)} ETH`);

  const approvals: Approval[] = [
    { token: meta.usdc.address, spender: CCTP_ADAPTER, symbol: "USDC", decimals: meta.usdc.decimals, bridge: "CCTP" },
    ...(OFT_TOKENS[id] ?? []),
  ];

  const status = await Promise.all(
    approvals.map(async (approval) => {
      const [bal, allowance] = await Promise.all([
        pub.readContract({ address: approval.token, abi: erc20Abi, functionName: "balanceOf", args: [account.address] }) as Promise<bigint>,
        pub.readContract({ address: approval.token, abi: erc20Abi, functionName: "allowance", args: [account.address, approval.spender] }) as Promise<bigint>,
      ]);
      return { approval, bal, allowance };
    }),
  );

  const pending = status.filter((s) => s.allowance < APPROVE_THRESHOLD);
  for (const s of status) {
    const tag = s.allowance >= APPROVE_THRESHOLD ? "already approved" : `approving ${s.approval.spender}…`;
    lines.push(`  ${s.approval.symbol} (${s.approval.bridge}): ${formatUnits(s.bal, s.approval.decimals)} — ${tag}`);
  }

  if (pending.length === 0) return lines;

  // Manage nonces locally: viem's auto-nonce reads `pending` per call, so back-to-back submissions
  // can race and collide. Pin a starting nonce and increment per tx so all approvals are in flight
  // immediately with distinct nonces.
  let nonce = await pub.getTransactionCount({ address: account.address, blockTag: "pending" });
  const submissions = pending.map(({ approval }) => {
    const txNonce = nonce++;
    return wallet
      .writeContract({
        address: approval.token,
        abi: erc20Abi,
        functionName: "approve",
        args: [approval.spender, maxUint256],
        nonce: txNonce,
      })
      .then(async (hash) => {
        await pub.waitForTransactionReceipt({ hash });
        return { approval, hash };
      });
  });
  const results = await Promise.all(submissions);
  for (const { approval, hash } of results) {
    lines.push(`    ${approval.symbol} approved (${hash})`);
  }
  return lines;
}

async function run() {
  console.log(`Relayer: ${account.address}\n`);
  const sections = await Promise.all(SUPPORTED_CHAIN_IDS.map(processChain));
  for (const lines of sections) {
    for (const line of lines) console.log(line);
    console.log("");
  }
  console.log("Done. The relayer can now fill on every configured destination.");
}

run().catch((e) => {
  console.error(e);
  process.exit(1);
});
