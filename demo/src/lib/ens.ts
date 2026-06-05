import { createPublicClient, http } from "viem";
import { mainnet } from "viem/chains";

/**
 * Standalone Ethereum-mainnet client used ONLY for ENS resolution in the nav. ENS records live on L1,
 * which is not one of the bridge's chains — keeping this client separate from the wagmi config avoids
 * adding mainnet to the RainbowKit network switcher.
 *
 * Defaults to a public RPC (ENS data is public, so there's no secret to protect and the server-side
 * Alchemy key stays server-side). Override with `NEXT_PUBLIC_ENS_RPC_URL` for a dedicated endpoint.
 */
const ENS_RPC_URL = process.env.NEXT_PUBLIC_ENS_RPC_URL || "https://ethereum-rpc.publicnode.com";

export const ensClient = createPublicClient({ chain: mainnet, transport: http(ENS_RPC_URL) });
