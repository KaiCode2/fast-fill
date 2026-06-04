import { existsSync, readFileSync } from "node:fs";
import { resolve } from "node:path";
import {
  createPublicClient,
  createWalletClient,
  encodeAbiParameters,
  formatUnits,
  getAddress,
  http,
  keccak256,
  pad,
  parseEventLogs,
  type Address,
  type Hex,
} from "viem";
import { privateKeyToAccount } from "viem/accounts";
import { arbitrum, base, optimism } from "viem/chains";

loadRootEnv();

const ALCHEMY_API_KEY = mustEnv("ALCHEMY_API_KEY");
const PRIVATE_KEY = mustEnv("DEMO_PRIVATE_KEY") as Hex;

const ADAPTER = "0x9FA37faBfA1Fd31Afe5A5F93e1c4Cd986b27bA75" as Address;
const EXECUTOR = "0xAFc7bBc0B5fD7A4d9b936349cfE991e5bC6E2a80" as Address;
const IRIS_BASE = process.env.CIRCLE_IRIS_BASE ?? "https://iris-api.circle.com";
const ZERO_HASH = "0x0000000000000000000000000000000000000000000000000000000000000000";

const FINALITY_FAST = 1000;
const DELIVERY_WINDOW = 3600n;
const DISCOUNT_RATE = 0n;
const EMPTY_EXEC = { gasLimit: 0n, data: "0x" as Hex };

const account = privateKeyToAccount(PRIVATE_KEY);

const chains = {
  8453: {
    name: "Base",
    chain: base,
    rpc: `https://base-mainnet.g.alchemy.com/v2/${ALCHEMY_API_KEY}`,
    cctpDomain: 6,
    usdc: "0x833589fCD6eDb6E08f4c7C32D4f71b54bdA02913" as Address,
  },
  10: {
    name: "Optimism",
    chain: optimism,
    rpc: `https://opt-mainnet.g.alchemy.com/v2/${ALCHEMY_API_KEY}`,
    cctpDomain: 2,
    usdc: "0x0b2C639c533813f4Aa9D7837CAf62653d097Ff85" as Address,
  },
  42161: {
    name: "Arbitrum",
    chain: arbitrum,
    rpc: `https://arb-mainnet.g.alchemy.com/v2/${ALCHEMY_API_KEY}`,
    cctpDomain: 3,
    usdc: "0xaf88d065e77c8cC2239327C5EDb3A432268e5831" as Address,
  },
} as const;

type ChainId = keyof typeof chains;

const erc20Abi = [
  {
    type: "function",
    name: "allowance",
    stateMutability: "view",
    inputs: [
      { name: "owner", type: "address" },
      { name: "spender", type: "address" },
    ],
    outputs: [{ name: "", type: "uint256" }],
  },
  {
    type: "function",
    name: "approve",
    stateMutability: "nonpayable",
    inputs: [
      { name: "spender", type: "address" },
      { name: "amount", type: "uint256" },
    ],
    outputs: [{ name: "", type: "bool" }],
  },
  {
    type: "function",
    name: "balanceOf",
    stateMutability: "view",
    inputs: [{ name: "account", type: "address" }],
    outputs: [{ name: "", type: "uint256" }],
  },
] as const;

const orderTuple = {
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

type Order = {
  bridgeType: number;
  srcChainId: number;
  dstChainId: number;
  sender: Hex;
  recipient: Hex;
  inputToken: Hex;
  outputToken: Hex;
  inputAmount: bigint;
  outputAmount: bigint;
  nonce: bigint;
  startTime: bigint;
  expectedDeliveryTime: bigint;
  discountRate: bigint;
  baseFee: bigint;
  callbackGasLimit: bigint;
  hookData: Hex;
};

const cctpAdapterAbi = [
  {
    type: "function",
    name: "initiateCCTP",
    stateMutability: "nonpayable",
    inputs: [
      { name: "dstChainId", type: "uint32" },
      { name: "recipient", type: "bytes32" },
      { name: "inputAmount", type: "uint256" },
      { name: "maxFee", type: "uint256" },
      { name: "mintFee", type: "uint256" },
      { name: "minFinalityThreshold", type: "uint32" },
      { name: "deliveryWindow", type: "uint64" },
      { name: "discountRate", type: "uint256" },
      { name: "baseFee", type: "uint256" },
      {
        name: "exec",
        type: "tuple",
        components: [
          { name: "gasLimit", type: "uint64" },
          { name: "data", type: "bytes" },
        ],
      },
    ],
    outputs: [
      { name: "orderId", type: "bytes32" },
      { name: "nonce", type: "uint64" },
    ],
  },
  {
    type: "function",
    name: "fill",
    stateMutability: "nonpayable",
    inputs: [{ name: "order", ...orderTuple }],
    outputs: [{ name: "orderId", type: "bytes32" }],
  },
  {
    type: "function",
    name: "settle",
    stateMutability: "nonpayable",
    inputs: [
      { name: "message", type: "bytes" },
      { name: "attestation", type: "bytes" },
    ],
    outputs: [],
  },
  {
    type: "function",
    name: "getOrder",
    stateMutability: "view",
    inputs: [{ name: "orderId", type: "bytes32" }],
    outputs: [
      {
        name: "",
        type: "tuple",
        components: [
          { name: "filler", type: "address" },
          { name: "status", type: "uint8" },
          { name: "fillTime", type: "uint40" },
        ],
      },
    ],
  },
  {
    type: "event",
    name: "OrderCreated",
    inputs: [
      { name: "orderId", type: "bytes32", indexed: true },
      { name: "bridgeType", type: "uint8", indexed: false },
      { name: "sender", type: "address", indexed: true },
      { name: "dstChainId", type: "uint32", indexed: false },
      { name: "outputToken", type: "bytes32", indexed: false },
      { name: "outputAmount", type: "uint256", indexed: false },
      { name: "nonce", type: "uint64", indexed: false },
    ],
  },
  {
    type: "event",
    name: "OrderFilled",
    inputs: [
      { name: "orderId", type: "bytes32", indexed: true },
      { name: "filler", type: "address", indexed: true },
      { name: "payoutToRecipient", type: "uint256", indexed: false },
      { name: "feeToFiller", type: "uint256", indexed: false },
      { name: "fillTime", type: "uint40", indexed: false },
    ],
  },
  {
    type: "event",
    name: "OrderSettled",
    inputs: [
      { name: "orderId", type: "bytes32", indexed: true },
      { name: "filler", type: "address", indexed: true },
      { name: "arrivedAmount", type: "uint256", indexed: false },
      { name: "surplusToRecipient", type: "uint256", indexed: false },
    ],
  },
] as const;

const cctpExecutorAbi = [
  {
    type: "function",
    name: "execute",
    stateMutability: "nonpayable",
    inputs: [
      { name: "message", type: "bytes" },
      { name: "attestation", type: "bytes" },
    ],
    outputs: [],
  },
  {
    type: "event",
    name: "Executed",
    inputs: [
      { name: "id", type: "bytes32", indexed: true },
      { name: "relayer", type: "address", indexed: true },
      { name: "target", type: "address", indexed: true },
      { name: "mintFee", type: "uint256", indexed: false },
      { name: "forwarded", type: "uint256", indexed: false },
    ],
  },
] as const;

const flows = [
  {
    label: "direct-unfilled Base -> Arbitrum",
    src: 8453 as ChainId,
    dst: 42161 as ChainId,
    amount: 100_000n,
    maxFee: 10_000n,
    mintFee: 0n,
    baseFee: 0n,
    fill: false,
  },
  {
    label: "routed-unfilled Arbitrum -> Base",
    src: 42161 as ChainId,
    dst: 8453 as ChainId,
    amount: 100_000n,
    maxFee: 10_000n,
    mintFee: 1_000n,
    baseFee: 0n,
    fill: false,
  },
  {
    label: "routed-filled Optimism -> Arbitrum",
    src: 10 as ChainId,
    dst: 42161 as ChainId,
    amount: 100_000n,
    maxFee: 10_000n,
    mintFee: 1_000n,
    baseFee: 1_000n,
    fill: true,
  },
] as const;

async function main() {
  console.log(`deployer=${account.address}`);
  console.log(`adapter=${ADAPTER}`);
  console.log(`executor=${EXECUTOR}`);

  const filter = process.env.FLOW_FILTER?.toLowerCase();
  const selected = filter ? flows.filter((flow) => flow.label.toLowerCase().includes(filter)) : flows;
  if (selected.length === 0) throw new Error(`FLOW_FILTER matched no flows: ${filter}`);

  for (const flow of selected) {
    await runFlow(flow);
  }
}

async function runFlow(flow: (typeof flows)[number]) {
  const src = chains[flow.src];
  const dst = chains[flow.dst];
  console.log(`\n=== ${flow.label} ===`);
  console.log(
    `amount=${formatUsdc(flow.amount)} maxFee=${formatUsdc(flow.maxFee)} mintFee=${formatUsdc(flow.mintFee)} baseFee=${formatUsdc(flow.baseFee)}`,
  );

  await ensureAllowance(flow.src, src.usdc, ADAPTER, flow.amount);

  const initHash = await writeSimulated(flow.src, {
    address: ADAPTER,
    abi: cctpAdapterAbi,
    functionName: "initiateCCTP",
    args: [
      flow.dst,
      addressToBytes32(account.address),
      flow.amount,
      flow.maxFee,
      flow.mintFee,
      FINALITY_FAST,
      DELIVERY_WINDOW,
      DISCOUNT_RATE,
      flow.baseFee,
      EMPTY_EXEC,
    ],
  });
  console.log(`initiate submitted=${initHash}`);
  const initReceipt = await pub(flow.src).waitForTransactionReceipt({ hash: initHash });
  const block =
    initReceipt.blockHash && initReceipt.blockHash !== ZERO_HASH
      ? await pub(flow.src).getBlock({ blockHash: initReceipt.blockHash })
      : await pub(flow.src).getBlock({ blockNumber: initReceipt.blockNumber });
  const created = parseSingleEvent(initReceipt.logs, cctpAdapterAbi, "OrderCreated", ADAPTER);
  const order = buildOrder({
    flow,
    nonce: created.args.nonce,
    startTime: block.timestamp,
  });
  const computedOrderId = orderIdOf(order);
  if (computedOrderId.toLowerCase() !== created.args.orderId.toLowerCase()) {
    throw new Error(`orderId mismatch: event=${created.args.orderId} computed=${computedOrderId}`);
  }
  console.log(`initiate=${initHash}`);
  console.log(`orderId=${computedOrderId}`);

  if (flow.fill) {
    await ensureAllowance(flow.dst, dst.usdc, ADAPTER, order.outputAmount);
    const fillHash = await writeSimulated(flow.dst, {
      address: ADAPTER,
      abi: cctpAdapterAbi,
      functionName: "fill",
      args: [order],
    });
    const fillReceipt = await pub(flow.dst).waitForTransactionReceipt({ hash: fillHash });
    const filled = parseSingleEvent(fillReceipt.logs, cctpAdapterAbi, "OrderFilled", ADAPTER);
    await waitForOrderStatus(flow.dst, computedOrderId, 1);
    console.log(
      `fill=${fillHash} payout=${formatUsdc(filled.args.payoutToRecipient)} fee=${formatUsdc(filled.args.feeToFiller)}`,
    );
  }

  const att = await waitForAttestation(flow.src, initHash);

  if (flow.mintFee === 0n) {
    await expectRevert(
      flow.dst,
      {
        address: EXECUTOR,
        abi: cctpExecutorAbi,
        functionName: "execute",
        args: [att.message, att.attestation],
      },
      "executor rejected direct message",
    );
  } else {
    await expectRevert(
      flow.dst,
      {
        address: ADAPTER,
        abi: cctpAdapterAbi,
        functionName: "settle",
        args: [att.message, att.attestation],
      },
      "adapter rejected routed message",
    );
  }

  const settleHash =
    flow.mintFee === 0n
      ? await writeSimulated(flow.dst, {
          address: ADAPTER,
          abi: cctpAdapterAbi,
          functionName: "settle",
          args: [att.message, att.attestation],
        })
      : await writeSimulated(flow.dst, {
          address: EXECUTOR,
          abi: cctpExecutorAbi,
          functionName: "execute",
          args: [att.message, att.attestation],
        });
  console.log(`${flow.mintFee === 0n ? "settle" : "execute"} submitted=${settleHash}`);
  const settleReceipt = await pub(flow.dst).waitForTransactionReceipt({ hash: settleHash });
  const settled = parseSingleEvent(settleReceipt.logs, cctpAdapterAbi, "OrderSettled", ADAPTER);
  await waitForOrderStatus(flow.dst, computedOrderId, 2);

  if (flow.mintFee !== 0n) {
    const executed = parseSingleEvent(settleReceipt.logs, cctpExecutorAbi, "Executed", EXECUTOR);
    if (executed.args.mintFee !== flow.mintFee) {
      throw new Error(`executor mintFee mismatch: ${executed.args.mintFee} != ${flow.mintFee}`);
    }
    console.log(`execute=${settleHash} forwarded=${formatUsdc(executed.args.forwarded)}`);
  } else {
    console.log(`settle=${settleHash}`);
  }
  console.log(
    `settled arrived=${formatUsdc(settled.args.arrivedAmount)} surplus=${formatUsdc(settled.args.surplusToRecipient)} filler=${settled.args.filler}`,
  );

  await expectRevert(
    flow.dst,
    flow.mintFee === 0n
      ? {
          address: ADAPTER,
          abi: cctpAdapterAbi,
          functionName: "settle",
          args: [att.message, att.attestation],
        }
      : {
          address: EXECUTOR,
          abi: cctpExecutorAbi,
          functionName: "execute",
          args: [att.message, att.attestation],
        },
    "replay rejected after settlement",
  );
}

function buildOrder(args: { flow: (typeof flows)[number]; nonce: bigint; startTime: bigint }): Order {
  const { flow, nonce, startTime } = args;
  const src = chains[flow.src];
  const dst = chains[flow.dst];
  return {
    bridgeType: 0,
    srcChainId: flow.src,
    dstChainId: flow.dst,
    sender: addressToBytes32(account.address),
    recipient: addressToBytes32(account.address),
    inputToken: addressToBytes32(src.usdc),
    outputToken: addressToBytes32(dst.usdc),
    inputAmount: flow.amount,
    outputAmount: flow.amount - flow.maxFee - flow.mintFee,
    nonce,
    startTime,
    expectedDeliveryTime: startTime + DELIVERY_WINDOW,
    discountRate: DISCOUNT_RATE,
    baseFee: flow.baseFee,
    callbackGasLimit: 0n,
    hookData: "0x",
  };
}

function orderIdOf(order: Order): Hex {
  return keccak256(encodeAbiParameters([orderTuple], [order]));
}

async function ensureAllowance(chainId: ChainId, token: Address, spender: Address, amount: bigint) {
  const allowance = await pub(chainId).readContract({
    address: token,
    abi: erc20Abi,
    functionName: "allowance",
    args: [account.address, spender],
  });
  if (allowance >= amount) return;

  const approveAmount = amount * 20n;
  const hash = await writeSimulated(chainId, {
    address: token,
    abi: erc20Abi,
    functionName: "approve",
    args: [spender, approveAmount],
  });
  await pub(chainId).waitForTransactionReceipt({ hash });
  await waitForAllowance(chainId, token, spender, amount);
  console.log(`${chains[chainId].name} approve ${spender} for ${formatUsdc(approveAmount)} USDC: ${hash}`);
}

async function waitForAllowance(chainId: ChainId, token: Address, spender: Address, amount: bigint) {
  const deadline = Date.now() + 60_000;
  while (Date.now() < deadline) {
    const allowance = await pub(chainId).readContract({
      address: token,
      abi: erc20Abi,
      functionName: "allowance",
      args: [account.address, spender],
    });
    if (allowance >= amount) return;
    await sleep(2_000);
  }
  throw new Error(`${chains[chainId].name} allowance did not become visible`);
}

async function getOrder(chainId: ChainId, orderId: Hex) {
  return pub(chainId).readContract({
    address: ADAPTER,
    abi: cctpAdapterAbi,
    functionName: "getOrder",
    args: [orderId],
  });
}

async function waitForOrderStatus(chainId: ChainId, orderId: Hex, status: number) {
  const deadline = Date.now() + 60_000;
  while (Date.now() < deadline) {
    const rec = await getOrder(chainId, orderId);
    if (rec.status === status) return rec;
    await sleep(2_000);
  }
  const rec = await getOrder(chainId, orderId);
  throw new Error(`expected order ${orderId} status ${status}, got ${rec.status}`);
}

async function writeSimulated(chainId: ChainId, request: any): Promise<Hex> {
  const { request: simulated } = await pub(chainId).simulateContract({
    account,
    ...request,
  });
  return wallet(chainId).writeContract(simulated);
}

async function expectRevert(chainId: ChainId, request: any, label: string) {
  try {
    await pub(chainId).simulateContract({ account, ...request });
  } catch {
    console.log(label);
    return;
  }
  throw new Error(`${label}: simulation unexpectedly succeeded`);
}

async function waitForAttestation(srcChainId: ChainId, txHash: Hex) {
  const domain = chains[srcChainId].cctpDomain;
  const url = `${IRIS_BASE}/v2/messages/${domain}?transactionHash=${txHash}`;
  const deadline = Date.now() + 35 * 60_000;
  let lastStatus = "";

  while (Date.now() < deadline) {
    const res = await fetch(url, { cache: "no-store" });
    let status = "pending";
    let message: Hex | undefined;
    let attestation: Hex | undefined;
    let forwardState: string | undefined;

    if (res.status !== 404) {
      if (!res.ok) throw new Error(`Circle Iris HTTP ${res.status} for ${txHash}`);
      const json = (await res.json()) as {
        messages?: { status?: string; message?: Hex; attestation?: Hex; forwardState?: string }[];
      };
      const item = json.messages?.[0];
      status = item?.status ?? "pending";
      message = item?.message;
      attestation = item?.attestation;
      forwardState = item?.forwardState;
    }

    const printable = `${status}${forwardState ? `/${forwardState}` : ""}`;
    if (printable !== lastStatus) {
      console.log(`attestation ${txHash}: ${printable}`);
      lastStatus = printable;
    }

    if (status === "complete" && message && attestation) return { message, attestation };
    await sleep(10_000);
  }

  throw new Error(`timed out waiting for Circle attestation for ${txHash}`);
}

function parseSingleEvent(logs: any[], abi: any, eventName: string, address: Address) {
  const events = parseEventLogs({ abi, eventName, logs, strict: false });
  const event = events.find((e) => e.address.toLowerCase() === address.toLowerCase());
  if (!event) throw new Error(`missing ${eventName} event from ${address}`);
  return event as any;
}

function pub(chainId: ChainId) {
  const c = chains[chainId];
  return createPublicClient({ chain: c.chain, transport: http(c.rpc) });
}

function wallet(chainId: ChainId) {
  const c = chains[chainId];
  return createWalletClient({ account, chain: c.chain, transport: http(c.rpc) });
}

function addressToBytes32(addr: Address): Hex {
  return pad(getAddress(addr).toLowerCase() as Hex, { size: 32 });
}

function formatUsdc(value: bigint) {
  return formatUnits(value, 6);
}

function sleep(ms: number) {
  return new Promise((resolveSleep) => setTimeout(resolveSleep, ms));
}

function mustEnv(name: string) {
  const value = process.env[name];
  if (!value) throw new Error(`missing ${name}`);
  return value.replace(/^"|"$/g, "");
}

function loadRootEnv() {
  const path = resolve(process.cwd(), "..", ".env");
  if (!existsSync(path)) return;
  for (const line of readFileSync(path, "utf8").split(/\r?\n/)) {
    const trimmed = line.trim();
    if (!trimmed || trimmed.startsWith("#")) continue;
    const idx = trimmed.indexOf("=");
    if (idx === -1) continue;
    const key = trimmed.slice(0, idx).trim();
    let value = trimmed.slice(idx + 1).trim();
    value = value.replace(/^"|"$/g, "");
    process.env[key] ??= value;
  }
}

main().catch((err) => {
  console.error(err);
  process.exit(1);
});
