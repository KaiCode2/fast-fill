import "server-only";
import {
  BlockNotFoundError,
  createPublicClient,
  createWalletClient,
  http,
  type Account,
  type PublicClient,
  type WalletClient,
} from "viem";
import { privateKeyToAccount } from "viem/accounts";
import { REGISTRY, type SupportedChainId } from "@/lib/chains";
import { relayerPrivateKey, serverRpcUrl } from "./env";

let account: Account | undefined;
export function relayerAccount(): Account {
  if (!relayerPrivateKey) throw new Error("RELAYER_PRIVATE_KEY is not set");
  if (!account) account = privateKeyToAccount(relayerPrivateKey);
  return account;
}

const pubCache: Partial<Record<SupportedChainId, PublicClient>> = {};
const walletCache: Partial<Record<SupportedChainId, WalletClient>> = {};

export function pub(chainId: SupportedChainId): PublicClient {
  if (!pubCache[chainId]) {
    pubCache[chainId] = createPublicClient({
      chain: REGISTRY[chainId].chain,
      transport: http(serverRpcUrl(chainId)),
    });
  }
  return pubCache[chainId]!;
}

const sleep = (ms: number) => new Promise<void>((r) => setTimeout(r, ms));

/**
 * Fetch a block by number, tolerating the brief window where a load-balanced RPC has already
 * served the receipt from a node ahead of the one answering `eth_getBlockByNumber`. viem turns a
 * null block into a `BlockNotFoundError` ("Block at number … could not be found"); we retry only
 * that with an escalating backoff and rethrow anything else.
 */
export async function getBlockWithRetry(
  client: PublicClient,
  blockNumber: bigint,
  opts: { retries?: number; delayMs?: number } = {},
) {
  const retries = opts.retries ?? 6;
  const delayMs = opts.delayMs ?? 400;
  let lastErr: unknown;
  for (let i = 0; i <= retries; i++) {
    try {
      return await client.getBlock({ blockNumber });
    } catch (e) {
      if (!(e instanceof BlockNotFoundError)) throw e;
      lastErr = e;
      if (i < retries) await sleep(delayMs * (i + 1));
    }
  }
  throw lastErr;
}

export function wallet(chainId: SupportedChainId): WalletClient {
  if (!walletCache[chainId]) {
    walletCache[chainId] = createWalletClient({
      account: relayerAccount(),
      chain: REGISTRY[chainId].chain,
      transport: http(serverRpcUrl(chainId)),
    });
  }
  return walletCache[chainId]!;
}

/**
 * Serialize state-changing txs per chain so concurrent jobs don't race the relayer's nonce.
 * A per-chain promise chain: each call waits for the previous to settle before running.
 */
const chainLocks: Partial<Record<number, Promise<unknown>>> = {};
export async function withChainLock<T>(chainId: number, fn: () => Promise<T>): Promise<T> {
  const prev = chainLocks[chainId] ?? Promise.resolve();
  let release!: () => void;
  const gate = new Promise<void>((r) => (release = r));
  chainLocks[chainId] = prev.then(() => gate);
  await prev.catch(() => undefined);
  try {
    return await fn();
  } finally {
    release();
  }
}
