import { encodeAbiParameters, keccak256, type Address, type Hex } from "viem";
import { PERMIT2 } from "./chains";

/**
 * Off-chain construction of the Permit2 `PermitWitnessTransferFrom` typed data whose witness is the
 * fast-fill `OrderIntent`. Must match `src/libraries/PermitLib.sol` exactly:
 *   - field order/types identical to ORDER_INTENT_TYPEHASH,
 *   - `deliveryWindow` is RELATIVE seconds (not absolute timestamps),
 *   - bridgeParams = keccak256(abi.encode(maxFee, minFinalityThreshold, mintFee)) for CCTP,
 *                    keccak256(extraOptions) for OFT.
 * Validated against the literal contract strings by `scripts/parity-check.ts`.
 */

export function cctpBridgeParams(maxFee: bigint, minFinalityThreshold: number, mintFee: bigint): Hex {
  return keccak256(
    encodeAbiParameters(
      [{ type: "uint256" }, { type: "uint32" }, { type: "uint256" }],
      [maxFee, minFinalityThreshold, mintFee],
    ),
  );
}

export function oftBridgeParams(extraOptions: Hex): Hex {
  return keccak256(extraOptions);
}

export const PERMIT2_WITNESS_TYPES = {
  PermitWitnessTransferFrom: [
    { name: "permitted", type: "TokenPermissions" },
    { name: "spender", type: "address" },
    { name: "nonce", type: "uint256" },
    { name: "deadline", type: "uint256" },
    { name: "witness", type: "OrderIntent" },
  ],
  TokenPermissions: [
    { name: "token", type: "address" },
    { name: "amount", type: "uint256" },
  ],
  OrderIntent: [
    { name: "bridgeType", type: "uint8" },
    { name: "dstChainId", type: "uint32" },
    { name: "recipient", type: "bytes32" },
    { name: "inputAmount", type: "uint256" },
    { name: "outputAmount", type: "uint256" },
    { name: "deliveryWindow", type: "uint64" },
    { name: "discountRate", type: "uint256" },
    { name: "baseFee", type: "uint256" },
    { name: "bridgeParams", type: "bytes32" },
    { name: "hookDataHash", type: "bytes32" },
    { name: "callbackGasLimit", type: "uint64" },
  ],
} as const;

export interface OrderIntentInput {
  chainId: number;
  spender: Address; // the adapter
  token: Address; // input token pulled from the signer
  inputAmount: bigint;
  outputAmount: bigint;
  recipient: Hex; // bytes32
  bridgeType: number;
  dstChainId: number;
  deliveryWindow: bigint; // RELATIVE seconds
  discountRate: bigint;
  baseFee: bigint;
  bridgeParams: Hex;
  hookDataHash: Hex; // keccak256(hookData)
  callbackGasLimit: bigint;
  nonce: bigint; // Permit2 unordered nonce
  deadline: bigint;
}

export function buildOrderIntentTypedData(p: OrderIntentInput) {
  return {
    domain: { name: "Permit2", chainId: p.chainId, verifyingContract: PERMIT2 as Address },
    types: PERMIT2_WITNESS_TYPES,
    primaryType: "PermitWitnessTransferFrom" as const,
    message: {
      permitted: { token: p.token, amount: p.inputAmount },
      spender: p.spender,
      nonce: p.nonce,
      deadline: p.deadline,
      witness: {
        bridgeType: p.bridgeType,
        dstChainId: p.dstChainId,
        recipient: p.recipient,
        inputAmount: p.inputAmount,
        outputAmount: p.outputAmount,
        deliveryWindow: p.deliveryWindow,
        discountRate: p.discountRate,
        baseFee: p.baseFee,
        bridgeParams: p.bridgeParams,
        hookDataHash: p.hookDataHash,
        callbackGasLimit: p.callbackGasLimit,
      },
    },
  };
}

/** A fresh Permit2 unordered nonce (256-bit random). */
export function randomPermit2Nonce(): bigint {
  const bytes = new Uint8Array(32);
  crypto.getRandomValues(bytes);
  let n = 0n;
  for (const b of bytes) n = (n << 8n) | BigInt(b);
  return n;
}
