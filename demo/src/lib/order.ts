import { encodeAbiParameters, getAddress, keccak256, pad, type Address, type Hex } from "viem";

/**
 * Off-chain mirror of `src/libraries/OrderLib.sol`. The single load-bearing invariant of the whole
 * protocol is `orderId = keccak256(abi.encode(order))`, computed identically at source-encode, at
 * fill, and at settle. `ORDER_TUPLE` + `encodeOrder` must be byte-identical to the Solidity struct
 * (same field order, same types) or a relayer's recomputed orderId will never match what settles.
 */

export const BRIDGE_CCTP = 0;
export const BRIDGE_OFT = 1;

export interface Order {
  bridgeType: number; // uint8  — 0 CCTP, 1 OFT
  srcChainId: number; // uint32
  dstChainId: number; // uint32
  sender: Hex; // bytes32
  recipient: Hex; // bytes32
  inputToken: Hex; // bytes32
  outputToken: Hex; // bytes32
  inputAmount: bigint; // uint256
  outputAmount: bigint; // uint256
  nonce: bigint; // uint64
  startTime: bigint; // uint64
  expectedDeliveryTime: bigint; // uint64
  discountRate: bigint; // uint256
  baseFee: bigint; // uint256
  callbackGasLimit: bigint; // uint64 — gas for the recipient's onFastFill hook (0 = none)
  hookData: Hex; // bytes — destination-execution payload ("0x" = deliver funds only)
}

export const ORDER_TUPLE = {
  type: "tuple",
  components: [
    { name: "bridgeType", type: "uint8" },
    { name: "srcChainId", type: "uint32" },
    { name: "dstChainId", type: "uint32" },
    { name: "sender", type: "bytes32" },
    { name: "recipient", type: "bytes32" },
    { name: "inputToken", type: "bytes32" },
    { name: "outputToken", type: "bytes32" },
    { name: "inputAmount", type: "uint256" },
    { name: "outputAmount", type: "uint256" },
    { name: "nonce", type: "uint64" },
    { name: "startTime", type: "uint64" },
    { name: "expectedDeliveryTime", type: "uint64" },
    { name: "discountRate", type: "uint256" },
    { name: "baseFee", type: "uint256" },
    { name: "callbackGasLimit", type: "uint64" },
    { name: "hookData", type: "bytes" },
  ],
} as const;

/** `abi.encode(order)` — the exact bytes the bridge carries as hookData / composeMsg. */
export function encodeOrder(order: Order): Hex {
  return encodeAbiParameters([ORDER_TUPLE], [order]);
}

/** `keccak256(abi.encode(order))` — the canonical order id. */
export function orderIdOf(order: Order): Hex {
  return keccak256(encodeOrder(order));
}

/** Left-pad a 20-byte address into a bytes32, matching `AddressCast.toBytes32`. */
export function addressToBytes32(addr: Address): Hex {
  return pad(addr.toLowerCase() as Hex, { size: 32 });
}

/** Take the low 20 bytes of a bytes32 back to a checksummed address. */
export function bytes32ToAddress(b: Hex): Address {
  return getAddress(`0x${b.slice(-40)}`);
}

/** Serialize an Order to JSON-safe strings (bigint → decimal string) for API transport. */
export function serializeOrder(order: Order): Record<string, string | number> {
  return {
    bridgeType: order.bridgeType,
    srcChainId: order.srcChainId,
    dstChainId: order.dstChainId,
    sender: order.sender,
    recipient: order.recipient,
    inputToken: order.inputToken,
    outputToken: order.outputToken,
    inputAmount: order.inputAmount.toString(),
    outputAmount: order.outputAmount.toString(),
    nonce: order.nonce.toString(),
    startTime: order.startTime.toString(),
    expectedDeliveryTime: order.expectedDeliveryTime.toString(),
    discountRate: order.discountRate.toString(),
    baseFee: order.baseFee.toString(),
    callbackGasLimit: order.callbackGasLimit.toString(),
    hookData: order.hookData,
  };
}

/** Inverse of `serializeOrder` — parse the API payload back into an Order with bigints. */
export function deserializeOrder(raw: Record<string, unknown>): Order {
  const bi = (k: string): bigint => BigInt(String(raw[k]));
  const num = (k: string): number => Number(raw[k]);
  const hex = (k: string): Hex => String(raw[k]) as Hex;
  return {
    bridgeType: num("bridgeType"),
    srcChainId: num("srcChainId"),
    dstChainId: num("dstChainId"),
    sender: hex("sender"),
    recipient: hex("recipient"),
    inputToken: hex("inputToken"),
    outputToken: hex("outputToken"),
    inputAmount: bi("inputAmount"),
    outputAmount: bi("outputAmount"),
    nonce: bi("nonce"),
    startTime: bi("startTime"),
    expectedDeliveryTime: bi("expectedDeliveryTime"),
    discountRate: bi("discountRate"),
    baseFee: bi("baseFee"),
    callbackGasLimit: bi("callbackGasLimit"),
    hookData: hex("hookData"),
  };
}
