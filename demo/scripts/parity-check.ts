/**
 * Decisive parity test for the Permit2 OrderIntent witness: independently reconstruct the digest
 * exactly as the contract + Permit2 do (mirroring PermitFork.t.sol `_signPermit2`), and assert it
 * equals what viem's `hashTypedData` produces from our typed-data object. If these match, a wallet
 * signing our typed data produces a signature the on-chain Permit2 will accept.
 */
import { concat, encodeAbiParameters, hashTypedData, keccak256, toBytes, type Hex } from "viem";
import { PERMIT2 } from "../src/lib/chains";
import { addressToBytes32 } from "../src/lib/order";
import { buildOrderIntentTypedData, cctpBridgeParams } from "../src/lib/permit2";

// Literal type strings from src/libraries/PermitLib.sol.
const ORDER_INTENT_TYPE =
  "OrderIntent(uint8 bridgeType,uint32 dstChainId,bytes32 recipient,uint256 inputAmount,uint256 outputAmount,uint64 deliveryWindow,uint256 discountRate,uint256 baseFee,bytes32 bridgeParams,bytes32 hookDataHash,uint64 callbackGasLimit)";
const ORDER_WITNESS_TYPE_STRING =
  "OrderIntent witness)" + ORDER_INTENT_TYPE + "TokenPermissions(address token,uint256 amount)";
const STUB = "PermitWitnessTransferFrom(TokenPermissions permitted,address spender,uint256 nonce,uint256 deadline,";
const FULL_TYPE = STUB + ORDER_WITNESS_TYPE_STRING;

const ORDER_INTENT_TYPEHASH = keccak256(toBytes(ORDER_INTENT_TYPE));
const FULL_TYPEHASH = keccak256(toBytes(FULL_TYPE));
const TOKEN_PERMISSIONS_TYPEHASH = keccak256(toBytes("TokenPermissions(address token,uint256 amount)"));
const EIP712_DOMAIN_TYPEHASH = keccak256(toBytes("EIP712Domain(string name,uint256 chainId,address verifyingContract)"));

function domainSeparator(chainId: number): Hex {
  return keccak256(
    encodeAbiParameters(
      [{ type: "bytes32" }, { type: "bytes32" }, { type: "uint256" }, { type: "address" }],
      [EIP712_DOMAIN_TYPEHASH, keccak256(toBytes("Permit2")), BigInt(chainId), PERMIT2],
    ),
  );
}

interface Sample {
  chainId: number;
  spender: Hex;
  token: Hex;
  inputAmount: bigint;
  outputAmount: bigint;
  recipient: Hex;
  bridgeType: number;
  dstChainId: number;
  deliveryWindow: bigint;
  discountRate: bigint;
  baseFee: bigint;
  bridgeParams: Hex;
  hookDataHash: Hex;
  callbackGasLimit: bigint;
  nonce: bigint;
  deadline: bigint;
}

function manualDigest(p: Sample): Hex {
  const witnessHash = keccak256(
    encodeAbiParameters(
      [
        { type: "bytes32" },
        { type: "uint8" },
        { type: "uint32" },
        { type: "bytes32" },
        { type: "uint256" },
        { type: "uint256" },
        { type: "uint64" },
        { type: "uint256" },
        { type: "uint256" },
        { type: "bytes32" },
        { type: "bytes32" },
        { type: "uint64" },
      ],
      [
        ORDER_INTENT_TYPEHASH,
        p.bridgeType,
        p.dstChainId,
        p.recipient,
        p.inputAmount,
        p.outputAmount,
        p.deliveryWindow,
        p.discountRate,
        p.baseFee,
        p.bridgeParams,
        p.hookDataHash,
        p.callbackGasLimit,
      ],
    ),
  );
  const tokenPermissionsHash = keccak256(
    encodeAbiParameters(
      [{ type: "bytes32" }, { type: "address" }, { type: "uint256" }],
      [TOKEN_PERMISSIONS_TYPEHASH, p.token, p.inputAmount],
    ),
  );
  const structHash = keccak256(
    encodeAbiParameters(
      [{ type: "bytes32" }, { type: "bytes32" }, { type: "address" }, { type: "uint256" }, { type: "uint256" }, { type: "bytes32" }],
      [FULL_TYPEHASH, tokenPermissionsHash, p.spender, p.nonce, p.deadline, witnessHash],
    ),
  );
  return keccak256(concat(["0x1901", domainSeparator(p.chainId), structHash]));
}

const sample: Sample = {
  chainId: 8453,
  spender: "0xC4a956Ec34BF7C0B07e4c84B72232E83879Ce1c0",
  token: "0x833589fCD6eDb6E08f4c7C32D4f71b54bdA02913",
  inputAmount: 1_000_000n,
  outputAmount: 999_800n,
  recipient: addressToBytes32("0xb9590000000000000000000000000000000bE530"),
  bridgeType: 0,
  dstChainId: 42_161,
  deliveryWindow: 600n,
  discountRate: 10_000_000_000_000n,
  baseFee: 0n,
  bridgeParams: cctpBridgeParams(200n, 1000),
  hookDataHash: keccak256("0x"),
  callbackGasLimit: 0n,
  nonce: 12345n,
  deadline: 1_900_000_000n,
};

const viemDigest = hashTypedData(buildOrderIntentTypedData(sample));
const expected = manualDigest(sample);

if (viemDigest.toLowerCase() !== expected.toLowerCase()) {
  console.error("[parity] MISMATCH");
  console.error("  viem  :", viemDigest);
  console.error("  manual:", expected);
  process.exit(1);
}
console.log("[parity] OK — viem typed-data digest matches the contract/Permit2 reconstruction.");
console.log("  digest:", viemDigest);
