import "server-only";
import { decodeFunctionData, parseEventLogs, type Address, type Hex } from "viem";
import { cctpAdapterAbi, oftAdapterAbi } from "@/lib/abis/generated";
import { isSupportedChain, type SupportedChainId } from "@/lib/chains";
import { addressToBytes32, BRIDGE_CCTP, orderIdOf, type Order } from "@/lib/order";
import { getToken } from "@/lib/tokens";
import { pub } from "./clients";
import { adapterAddressServer } from "./env";
import { assertOrderAllowed } from "./guards";

export interface VerifiedOrder {
  order: Order;
  orderId: Hex;
  bridgeType: number;
  srcTxHash: Hex;
  srcBlockNumber: bigint;
}

function configuredAdapters(): { addr: Address; bridgeType: number }[] {
  const out: { addr: Address; bridgeType: number }[] = [];
  for (const bt of [0, 1]) {
    try {
      out.push({ addr: adapterAddressServer(bt), bridgeType: bt });
    } catch {
      /* adapter not configured (e.g. no OFT) */
    }
  }
  return out;
}

/** Pull the initiate call out of the tx input, unwrapping a `multicall([selfPermit, initiate])`. */
function findInitiate(input: Hex, abi: typeof cctpAdapterAbi | typeof oftAdapterAbi) {
  const decoded = decodeFunctionData({ abi, data: input });
  if (decoded.functionName === "multicall") {
    const calls = (decoded.args as readonly [readonly Hex[]])[0];
    for (const c of calls) {
      try {
        const inner = decodeFunctionData({ abi, data: c });
        if (inner.functionName.startsWith("initiate")) return inner;
      } catch {
        /* not an adapter call */
      }
    }
    throw new Error("no initiate call found inside multicall");
  }
  if (decoded.functionName.startsWith("initiate")) return decoded;
  throw new Error(`unexpected source function: ${decoded.functionName}`);
}

/**
 * Reconstruct the full Order from a source initiate tx and PROVE its authenticity: the recomputed
 * `orderId` must equal the `orderId` emitted by OUR adapter's `OrderCreated` in that tx. A relayer
 * that fills any order failing this check is donating funds — so this is the one mandatory gate.
 */
export async function reconstructAndVerify(srcChainId: number, txHash: Hex): Promise<VerifiedOrder> {
  if (!isSupportedChain(srcChainId)) throw new Error(`unsupported source chain ${srcChainId}`);
  const src = srcChainId as SupportedChainId;
  const client = pub(src);

  const [receipt, tx] = await Promise.all([
    client.getTransactionReceipt({ hash: txHash }),
    client.getTransaction({ hash: txHash }),
  ]);
  if (receipt.status !== "success") throw new Error("source transaction reverted");

  // Find an OrderCreated emitted by one of our adapters (pins the source contract + bridge family).
  const adapters = configuredAdapters();
  const events = parseEventLogs({ abi: cctpAdapterAbi, eventName: "OrderCreated", logs: receipt.logs });
  const match = events
    .map((e) => ({ e, hit: adapters.find((a) => a.addr.toLowerCase() === (e.address as string).toLowerCase()) }))
    .find((x) => x.hit);
  if (!match || !match.hit) throw new Error("no OrderCreated from a known fast-fill adapter in this tx");

  const bridgeType = match.hit.bridgeType;
  const ev = match.e.args as {
    orderId: Hex;
    bridgeType: number;
    sender: Address;
    dstChainId: number;
    outputToken: Hex;
    outputAmount: bigint;
    nonce: bigint;
  };
  if (Number(ev.bridgeType) !== bridgeType) throw new Error("bridgeType mismatch (adapter vs event)");

  // Decode the initiate args. CCTP has an extra `mintFee` after `maxFee`; OFT keeps its original shape.
  const abi = bridgeType === BRIDGE_CCTP ? cctpAdapterAbi : oftAdapterAbi;
  const init = findInitiate(tx.input, abi);
  const a = init.args as readonly unknown[];
  const dstChainId = Number(a[0] as number | bigint);
  if (!isSupportedChain(dstChainId)) throw new Error(`unsupported destination chain ${dstChainId}`);
  const dst = dstChainId as SupportedChainId;

  const recipient = a[1] as Hex; // bytes32
  const inputAmount = a[2] as bigint;
  const outputAmount =
    bridgeType === BRIDGE_CCTP ? inputAmount - (a[3] as bigint) - (a[4] as bigint) : (a[3] as bigint);
  const deliveryWindow = (bridgeType === BRIDGE_CCTP ? a[6] : a[5]) as bigint;
  const discountRate = (bridgeType === BRIDGE_CCTP ? a[7] : a[6]) as bigint;
  const baseFee = (bridgeType === BRIDGE_CCTP ? a[8] : a[7]) as bigint;
  const exec = (bridgeType === BRIDGE_CCTP ? a[9] : a[8]) as { gasLimit: bigint; data: Hex }; // Execution

  const symbol = bridgeType === BRIDGE_CCTP ? "USDC" : "USDT0";
  const block = await client.getBlock({ blockNumber: receipt.blockNumber });

  const order: Order = {
    bridgeType,
    srcChainId: src,
    dstChainId: dst,
    sender: addressToBytes32(ev.sender),
    recipient,
    inputToken: addressToBytes32(getToken(src, symbol).address),
    outputToken: addressToBytes32(getToken(dst, symbol).address),
    inputAmount,
    outputAmount,
    nonce: ev.nonce,
    startTime: block.timestamp,
    expectedDeliveryTime: block.timestamp + deliveryWindow,
    discountRate,
    baseFee,
    callbackGasLimit: exec.gasLimit,
    hookData: exec.data,
  };

  const orderId = orderIdOf(order);
  if (orderId.toLowerCase() !== ev.orderId.toLowerCase()) {
    throw new Error("authenticity check failed: recomputed orderId does not match OrderCreated");
  }

  assertOrderAllowed(order);
  return { order, orderId, bridgeType, srcTxHash: txHash, srcBlockNumber: receipt.blockNumber };
}
