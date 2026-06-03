import { encodePacked, type Hex } from "viem";

/**
 * LayerZero v2 type-3 executor options, byte-identical to `OftForkE2E._composeOptions` (the
 * fork-proven encoding the real USD₮0 OFT accepts). Two executor options:
 *   - lzReceive gas  (worker 1, type 1, uint128 gas)            → gas for the dst mint (_lzReceive)
 *   - lzCompose gas  (worker 1, type 3, uint16 index=0, uint128 gas) → gas for our settle (lzCompose)
 *
 * The compose option is what makes the LZ executor auto-deliver settlement. These bytes are ALSO
 * what a sponsored OFT intent commits to (`bridgeParams = keccak256(extraOptions)`), so the client
 * and the relayer MUST call this with identical gas constants — keep them fixed, not per-order.
 */
export const DEFAULT_LZRECEIVE_GAS = 80_000n;
export const DEFAULT_COMPOSE_GAS = 200_000n;

export function buildExtraOptions(
  lzReceiveGas: bigint = DEFAULT_LZRECEIVE_GAS,
  composeGas: bigint = DEFAULT_COMPOSE_GAS,
): Hex {
  // TYPE_3 prefix (0x0003), then the two executor option TLVs.
  return encodePacked(
    ["uint16", "uint8", "uint16", "uint8", "uint128", "uint8", "uint16", "uint8", "uint16", "uint128"],
    [
      3, // options type 3
      1,
      17,
      1,
      lzReceiveGas, // executor lzReceive: worker 1, size 17, type 1, gas
      1,
      19,
      3,
      0,
      composeGas, // executor lzCompose: worker 1, size 19, type 3, index 0, gas
    ],
  );
}
