import { parseSignature, type Address, type Hex } from "viem";

/** EIP-2612 `permit` typed data for batching `selfPermit` + initiate in one multicall tx. */
export function buildPermit2612TypedData(args: {
  name: string;
  version: string;
  chainId: number;
  token: Address;
  owner: Address;
  spender: Address;
  value: bigint;
  nonce: bigint;
  deadline: bigint;
}) {
  return {
    domain: { name: args.name, version: args.version, chainId: args.chainId, verifyingContract: args.token },
    types: {
      Permit: [
        { name: "owner", type: "address" },
        { name: "spender", type: "address" },
        { name: "value", type: "uint256" },
        { name: "nonce", type: "uint256" },
        { name: "deadline", type: "uint256" },
      ],
    },
    primaryType: "Permit" as const,
    message: {
      owner: args.owner,
      spender: args.spender,
      value: args.value,
      nonce: args.nonce,
      deadline: args.deadline,
    },
  };
}

/** Split a 65-byte signature into the (v, r, s) `selfPermit` expects. */
export function splitSignature(sig: Hex): { v: number; r: Hex; s: Hex } {
  const parsed = parseSignature(sig);
  const v = parsed.v !== undefined ? Number(parsed.v) : (parsed.yParity ?? 0) + 27;
  return { v, r: parsed.r, s: parsed.s };
}
