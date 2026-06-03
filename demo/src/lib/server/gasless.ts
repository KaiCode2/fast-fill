import "server-only";
import type { Address, Hex } from "viem";
import { cctpAdapterAbi, oftAdapterAbi } from "@/lib/abis/generated";
import { oftAbi } from "@/lib/abis/oft";
import type { GaslessBridgeRequest } from "@/lib/api";
import { REGISTRY, type SupportedChainId } from "@/lib/chains";
import { buildExtraOptions } from "@/lib/lzOptions";
import { addressToBytes32, BRIDGE_CCTP, encodeOrder, type Order } from "@/lib/order";
import { getToken, isRouteSupported } from "@/lib/tokens";
import { pub, relayerAccount, wallet, withChainLock } from "./clients";
import { adapterAddressServer, MAX_TRANSFER_BASE_UNITS, OFT_COMPOSE_GAS, OFT_LZRECEIVE_GAS } from "./env";
import { registerJob } from "./relayer";
import { jobByPermit } from "./store";
import { reconstructAndVerify } from "./verify";

const nowSec = () => BigInt(Math.floor(Date.now() / 1000));

function preGuard(req: GaslessBridgeRequest) {
  const sym = req.bridgeType === BRIDGE_CCTP ? "USDC" : "USDT0";
  if (!isRouteSupported(sym, req.srcChainId, req.dstChainId)) throw new Error("route not allowed");
  if (BigInt(req.inputAmount) > MAX_TRANSFER_BASE_UNITS) throw new Error("amount exceeds demo cap");
  if (BigInt(req.outputAmount) <= 0n || BigInt(req.outputAmount) > BigInt(req.inputAmount)) {
    throw new Error("invalid output amount");
  }
}

/** Quote the LZ native fee for the sponsored OFT send (size-equivalent preview composeMsg). */
async function quoteOftFee(req: GaslessBridgeRequest, adapter: Address, extraOptions: Hex): Promise<bigint> {
  const src = req.srcChainId;
  const oft = REGISTRY[src].usdt0Oft!;
  const preview: Order = {
    bridgeType: 1,
    srcChainId: src,
    dstChainId: req.dstChainId,
    sender: addressToBytes32(req.from as Address),
    recipient: req.recipient,
    inputToken: addressToBytes32(getToken(src, "USDT0").address),
    outputToken: addressToBytes32(getToken(req.dstChainId, "USDT0").address),
    inputAmount: BigInt(req.inputAmount),
    outputAmount: BigInt(req.outputAmount),
    nonce: 0n,
    startTime: nowSec(),
    expectedDeliveryTime: nowSec() + BigInt(req.deliveryWindow),
    discountRate: BigInt(req.discountRate),
    baseFee: BigInt(req.baseFee),
    callbackGasLimit: BigInt(req.callbackGasLimit ?? "0"),
    hookData: (req.hookData ?? "0x") as Hex,
  };
  const sendParam = {
    dstEid: REGISTRY[req.dstChainId].lzEid,
    to: addressToBytes32(adapter),
    amountLD: BigInt(req.inputAmount),
    minAmountLD: BigInt(req.outputAmount),
    extraOptions,
    composeMsg: encodeOrder(preview),
    oftCmd: "0x" as Hex,
  };
  const fee = (await pub(src).readContract({
    address: oft,
    abi: oftAbi,
    functionName: "quoteSend",
    args: [sendParam, false],
  })) as { nativeFee: bigint };
  return (fee.nativeFee * 12n) / 10n;
}

/** Relayer submits the user's signed Permit2 intent on the source chain, then enqueues the fill. */
export async function submitGasless(req: GaslessBridgeRequest): Promise<{ orderId: Hex; srcTxHash: Hex }> {
  const dup = jobByPermit(req.from, req.permit.nonce);
  if (dup?.srcTxHash) return { orderId: dup.orderId, srcTxHash: dup.srcTxHash };
  preGuard(req);

  const src = req.srcChainId as SupportedChainId;
  const adapter = adapterAddressServer(req.bridgeType);
  const permit = {
    nonce: BigInt(req.permit.nonce),
    deadline: BigInt(req.permit.deadline),
    signature: req.permit.signature,
  };
  const exec = { gasLimit: BigInt(req.callbackGasLimit ?? "0"), data: (req.hookData ?? "0x") as Hex };

  let txHash: Hex;
  if (req.bridgeType === BRIDGE_CCTP) {
    txHash = await withChainLock(src, async () => {
      const { request } = await pub(src).simulateContract({
        account: relayerAccount(),
        address: adapter,
        abi: cctpAdapterAbi,
        functionName: "initiateCCTPFor",
        args: [
          req.dstChainId,
          req.recipient,
          BigInt(req.inputAmount),
          BigInt(req.maxFee ?? "0"),
          req.minFinalityThreshold ?? 1000,
          BigInt(req.deliveryWindow),
          BigInt(req.discountRate),
          BigInt(req.baseFee),
          exec,
          req.from,
          permit,
        ],
      });
      const h = await wallet(src).writeContract(request);
      await pub(src).waitForTransactionReceipt({ hash: h });
      return h;
    });
  } else {
    const extraOptions = buildExtraOptions(
      BigInt(req.lzReceiveGas ?? OFT_LZRECEIVE_GAS),
      BigInt(req.composeGas ?? OFT_COMPOSE_GAS),
    );
    const value = await quoteOftFee(req, adapter, extraOptions);
    txHash = await withChainLock(src, async () => {
      const { request } = await pub(src).simulateContract({
        account: relayerAccount(),
        address: adapter,
        abi: oftAdapterAbi,
        functionName: "initiateOFTFor",
        args: [
          req.dstChainId,
          req.recipient,
          BigInt(req.inputAmount),
          BigInt(req.outputAmount),
          extraOptions,
          BigInt(req.deliveryWindow),
          BigInt(req.discountRate),
          BigInt(req.baseFee),
          exec,
          req.from,
          permit,
        ],
        value,
      });
      const h = await wallet(src).writeContract(request);
      await pub(src).waitForTransactionReceipt({ hash: h });
      return h;
    });
  }

  const verified = await reconstructAndVerify(src, txHash);
  const job = registerJob(verified, Boolean(req.forwarding), { from: req.from, permitNonce: req.permit.nonce });
  return { orderId: job.orderId, srcTxHash: txHash };
}
