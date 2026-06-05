import { encodeAbiParameters, type Address, type Hex } from "viem";
import type { SupportedChainId } from "./chains";

/**
 * Destination-execution hooks (PR #17). A hook is an on-chain contract the fast-fill adapter delivers
 * the bridged token to and then calls `onFastFill(orderId, token, amount, hookData)` on, in the same
 * atomic frame. To use a hook the order's `recipient` is set to the hook address and the real
 * beneficiary + params are encoded inside `hookData`. If the hook reverts, the protocol's
 * revert-to-redirect rule returns the original token to the beneficiary, so funds are never stuck.
 *
 * Addresses are the mainnet deploys from `DEPLOYMENTS.md` (Base · Optimism · Arbitrum). The hooks
 * embed their protocol dependency (Uniswap router / Aave pool) as an immutable, so OP and Arbitrum
 * share an address (same dependency) and Base differs. `encode*HookData` must byte-match the Solidity
 * `abi.decode` in `src/hooks/AaveDepositHook.sol` and `src/hooks/UniswapSwapHook.sol`.
 */

export type HookKind = "none" | "aave" | "uniswap";

const AAVE_DEPOSIT_HOOK: Record<SupportedChainId, Address> = {
  8453: "0xBE30475CaEEd5003541DbAA8973bb01bA8433DC3",
  10: "0xA0eCA1b76ff575B4031c510862f1024deFEEE321",
  42161: "0xA0eCA1b76ff575B4031c510862f1024deFEEE321",
};

const UNISWAP_SWAP_HOOK: Record<SupportedChainId, Address> = {
  8453: "0xDeAF6072b2774a49688Fd09817Be9FBFbdE2835e",
  10: "0x913FC613BE7a603Dc222Bce1997Ae28Fd7c48665",
  42161: "0x913FC613BE7a603Dc222Bce1997Ae28Fd7c48665",
};

/** The deployed hook address for a kind on a chain (undefined for "none" or an undeployed chain). */
export function hookAddress(kind: HookKind, chainId: SupportedChainId): Address | undefined {
  if (kind === "aave") return AAVE_DEPOSIT_HOOK[chainId];
  if (kind === "uniswap") return UNISWAP_SWAP_HOOK[chainId];
  return undefined;
}

/**
 * Gas budgeted for the hook's `onFastFill` callback. The integration tests run both actions in ~500k;
 * the protocol caps `callbackGasLimit` at 5,000,000.
 */
export const HOOK_CALLBACK_GAS: Record<Exclude<HookKind, "none">, bigint> = {
  aave: 500_000n,
  uniswap: 500_000n,
};

/** Human label for a hook kind (UI + history). */
export const HOOK_LABEL: Record<HookKind, string> = {
  none: "None",
  aave: "Deposit to Aave",
  uniswap: "Swap via Uniswap",
};

/** `AaveDepositHook`: `abi.encode(address user, uint16 referralCode)`. Supplies the delivered token to Aave V3 on behalf of `user`. */
export function encodeAaveHookData(user: Address, referralCode = 0): Hex {
  return encodeAbiParameters([{ type: "address" }, { type: "uint16" }], [user, referralCode]);
}

/** `UniswapSwapHook`: `abi.encode(address user, address tokenOut, uint24 poolFee, uint256 amountOutMinimum, uint160 sqrtPriceLimitX96)`. */
export function encodeUniswapHookData(args: {
  user: Address;
  tokenOut: Address;
  poolFee: number; // 100 | 500 | 3000 | 10000
  amountOutMinimum: bigint; // in tokenOut base units
  sqrtPriceLimitX96?: bigint; // 0 = no price limit (rely on amountOutMinimum)
}): Hex {
  return encodeAbiParameters(
    [{ type: "address" }, { type: "address" }, { type: "uint24" }, { type: "uint256" }, { type: "uint160" }],
    [args.user, args.tokenOut, args.poolFee, args.amountOutMinimum, args.sqrtPriceLimitX96 ?? 0n],
  );
}
