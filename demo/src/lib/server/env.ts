import "server-only";
import type { Address, Hex } from "viem";
import { alchemyRpcUrl, defaultRpcUrl, DEPLOYED, type SupportedChainId } from "@/lib/chains";

/** Server-only configuration. The relayer key and private RPCs are read ONLY here. */

export const relayerPrivateKey = process.env.RELAYER_PRIVATE_KEY as Hex | undefined;
export const relayerConfigured = Boolean(relayerPrivateKey);

/** Server prefers a private ALCHEMY_API_KEY, falling back to the public NEXT_PUBLIC one, then default. */
const alchemyApiKey = process.env.ALCHEMY_API_KEY ?? process.env.NEXT_PUBLIC_ALCHEMY_API_KEY;
export function serverRpcUrl(chainId: SupportedChainId): string {
  return alchemyApiKey ? alchemyRpcUrl(chainId, alchemyApiKey) : defaultRpcUrl(chainId);
}

export function adapterAddressServer(bridgeType: number): Address {
  const env = bridgeType === 0 ? process.env.NEXT_PUBLIC_CCTP_ADAPTER : process.env.NEXT_PUBLIC_OFT_ADAPTER;
  return (env ?? (bridgeType === 0 ? DEPLOYED.cctpAdapter : DEPLOYED.oftAdapter)) as Address;
}

/** Hard per-transfer cap in token base units (both tokens are 6dp → 5_000_000 = 5.00). */
export const MAX_TRANSFER_BASE_UNITS = BigInt(process.env.RELAYER_MAX_BASE_UNITS ?? "5000000");

/** Only fill when the quoted premium is at least this (base units). 0 = always fill (demo default). */
export const MIN_FEE_TO_FILL = BigInt(process.env.RELAYER_MIN_FEE ?? "0");

/** Source confirmations to wait before filling a self-submitted burn (reorg safety on L2s). */
export const SOURCE_CONFIRMATIONS = BigInt(process.env.RELAYER_SRC_CONFIRMATIONS ?? "1");

export const CIRCLE_IRIS_BASE = process.env.CIRCLE_IRIS_BASE ?? "https://iris-api.circle.com";
export const LZ_SCAN_BASE = process.env.LZ_SCAN_BASE ?? "https://scan.layerzero-api.com/v1";

/** Default LayerZero compose gas the relayer funds for sponsored OFT sends (matches lzOptions). */
export const OFT_LZRECEIVE_GAS = BigInt(process.env.RELAYER_OFT_LZRECEIVE_GAS ?? "80000");
export const OFT_COMPOSE_GAS = BigInt(process.env.RELAYER_OFT_COMPOSE_GAS ?? "200000");
