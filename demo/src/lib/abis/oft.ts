/**
 * The bits of a LayerZero v2 OFT (USD₮0) we touch directly: quoting the native messaging fee and
 * reading token/approval metadata. The adapter performs the actual `send`; we call `quoteSend`
 * against the real OFT to size `msg.value`.
 */
const SEND_PARAM = {
  type: "tuple",
  name: "_sendParam",
  components: [
    { name: "dstEid", type: "uint32" },
    { name: "to", type: "bytes32" },
    { name: "amountLD", type: "uint256" },
    { name: "minAmountLD", type: "uint256" },
    { name: "extraOptions", type: "bytes" },
    { name: "composeMsg", type: "bytes" },
    { name: "oftCmd", type: "bytes" },
  ],
} as const;

const MESSAGING_FEE = {
  type: "tuple",
  components: [
    { name: "nativeFee", type: "uint256" },
    { name: "lzTokenFee", type: "uint256" },
  ],
} as const;

export const oftAbi = [
  {
    type: "function",
    name: "quoteSend",
    stateMutability: "view",
    inputs: [SEND_PARAM, { name: "_payInLzToken", type: "bool" }],
    outputs: [MESSAGING_FEE],
  },
  {
    type: "function",
    name: "token",
    stateMutability: "view",
    inputs: [],
    outputs: [{ name: "", type: "address" }],
  },
  {
    type: "function",
    name: "approvalRequired",
    stateMutability: "view",
    inputs: [],
    outputs: [{ name: "", type: "bool" }],
  },
] as const;
