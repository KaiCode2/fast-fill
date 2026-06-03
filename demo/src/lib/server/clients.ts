import "server-only";
import {
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
