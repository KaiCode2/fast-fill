/**
 * Minimal Permit2 surface. We never call `permitWitnessTransferFrom` off-chain (the adapter does,
 * on-chain); off-chain we only read the unordered-nonce bitmap to pick an unused nonce, and the
 * domain separator for sanity. Signing uses viem's typed-data (domain built from {name, chainId,
 * verifyingContract} — Permit2 has no version field).
 */
export const permit2Abi = [
  {
    type: "function",
    name: "nonceBitmap",
    stateMutability: "view",
    inputs: [
      { name: "owner", type: "address" },
      { name: "wordPos", type: "uint256" },
    ],
    outputs: [{ name: "", type: "uint256" }],
  },
  {
    type: "function",
    name: "DOMAIN_SEPARATOR",
    stateMutability: "view",
    inputs: [],
    outputs: [{ name: "", type: "bytes32" }],
  },
] as const;
