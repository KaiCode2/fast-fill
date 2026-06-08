import "server-only";
import type { GaslessBridgeRequest, PricingQuoteRequest, PricingQuoteResponse } from "@/lib/api";
import { FINALITY_FAST, FINALITY_FINALIZED, outputAmountOf } from "@/lib/bridge";
import { isSupportedChain, REGISTRY, type SupportedChainId } from "@/lib/chains";
import {
  bufferedCctpMaxFee,
  bufferedGasCostToken,
  CCTP_DIRECT_SETTLE_GAS,
  cctpProtocolFee,
  gasBackedFees,
  opportunityCostQuote,
  OPPORTUNITY_APR_BPS,
  STANDALONE_TX_BASE_GAS,
} from "@/lib/gasPricing";
import { BRIDGE_CCTP } from "@/lib/order";
import { bridgeTypeForToken, getToken, isRouteSupported } from "@/lib/tokens";
import { pub } from "./clients";
import { CIRCLE_IRIS_BASE, MIN_FEE_TO_FILL } from "./env";

interface CircleForwardFee {
  low?: number | string;
  med?: number | string;
  high?: number | string;
}

interface CircleFeeRow {
  finalityThreshold: number;
  minimumFee: number | string;
  forwardFee?: CircleForwardFee;
}

let ethPriceCache: { price: bigint; at: number } | undefined;

function envInt(name: string, fallback: number): number {
  const raw = process.env[name];
  if (!raw) return fallback;
  const n = Number(raw);
  return Number.isFinite(n) && n > 0 ? Math.round(n) : fallback;
}

async function ethUsd(): Promise<bigint> {
  const ttlMs = envInt("RELAYER_ETH_PRICE_TTL_SECS", 4 * 3600) * 1000;
  if (ethPriceCache && Date.now() - ethPriceCache.at < ttlMs) return ethPriceCache.price;

  const fallback = BigInt(envInt("RELAYER_ETH_PRICE_USD", 3000));
  const url =
    process.env.RELAYER_ETH_PRICE_URL ??
    "https://api.coingecko.com/api/v3/simple/price?ids=ethereum&vs_currencies=usd";
  try {
    const res = await fetch(url, {
      cache: "no-store",
      headers: { "user-agent": "fast-fill-demo/0.1" },
    });
    if (!res.ok) throw new Error(`eth price http ${res.status}`);
    const json = (await res.json()) as { ethereum?: { usd?: number } };
    const usd = json.ethereum?.usd;
    if (!usd || usd <= 0) throw new Error("unexpected eth price response");
    const price = BigInt(Math.round(usd));
    ethPriceCache = { price, at: Date.now() };
    return price;
  } catch {
    if (ethPriceCache) return ethPriceCache.price;
    return fallback;
  }
}

async function circleFees(src: SupportedChainId, dst: SupportedChainId, forward: boolean): Promise<CircleFeeRow[]> {
  const srcDomain = REGISTRY[src].cctpDomain;
  const dstDomain = REGISTRY[dst].cctpDomain;
  const qs = forward ? "?forward=true" : "";
  const url = `${CIRCLE_IRIS_BASE.replace(/\/$/, "")}/v2/burn/USDC/fees/${srcDomain}/${dstDomain}${qs}`;
  const res = await fetch(url, { cache: "no-store", headers: { accept: "application/json" } });
  if (!res.ok) throw new Error(`Circle fee quote failed (${res.status})`);
  const json = (await res.json()) as CircleFeeRow[];
  if (!Array.isArray(json)) throw new Error("unexpected Circle fee response");
  return json;
}

function feeRow(rows: CircleFeeRow[], finality: number): CircleFeeRow {
  const row = rows.find((r) => Number(r.finalityThreshold) === finality);
  if (!row) throw new Error(`Circle fee row missing for finality ${finality}`);
  return row;
}

function forwardFeeMed(row: CircleFeeRow | undefined): bigint {
  const med = row?.forwardFee?.med;
  return med == null ? 0n : BigInt(med);
}

function signedSavings(reference: bigint, actual: bigint): string {
  return (reference - actual).toString();
}

function parseQuoteRequest(req: PricingQuoteRequest) {
  if (!isSupportedChain(req.srcChainId) || !isSupportedChain(req.dstChainId)) throw new Error("unsupported chain");
  if (req.bridgeType !== bridgeTypeForToken(req.token)) throw new Error("bridge/token mismatch");
  if (!isRouteSupported(req.token, req.srcChainId, req.dstChainId)) throw new Error("route not allowed");
  const amount = BigInt(req.amount);
  const deliveryWindow = BigInt(req.deliveryWindow);
  const callbackGasLimit = BigInt(req.callbackGasLimit);
  if (amount <= 0n) throw new Error("invalid amount");
  if (deliveryWindow <= 0n) throw new Error("invalid delivery window");
  if (callbackGasLimit < 0n) throw new Error("invalid callback gas limit");
  return { amount, deliveryWindow, callbackGasLimit };
}

export async function quoteBridgePricing(req: PricingQuoteRequest): Promise<PricingQuoteResponse> {
  const { amount, deliveryWindow, callbackGasLimit } = parseQuoteRequest(req);
  const decimals = getToken(req.dstChainId, req.token).decimals;

  const [gasPriceWei, ethUsdNow] = await Promise.all([pub(req.dstChainId).getGasPrice(), ethUsd()]);
  const gasFees = gasBackedFees({
    bridgeType: req.bridgeType,
    relayMint: req.bridgeType === BRIDGE_CCTP && req.relayMint,
    callbackGasLimit,
    gasPriceWei,
    ethUsd: ethUsdNow,
    decimals,
  });

  let maxFee = 0n;
  let protocolFee = 0n;
  let cctpMinimumFeeBps: string | undefined;
  let comparison: PricingQuoteResponse["comparison"];

  if (req.bridgeType === BRIDGE_CCTP) {
    const [fees, forwardFees] = await Promise.all([
      circleFees(req.srcChainId, req.dstChainId, false),
      circleFees(req.srcChainId, req.dstChainId, true),
    ]);
    const selectedRow = feeRow(fees, req.finality);
    cctpMinimumFeeBps = String(selectedRow.minimumFee);
    protocolFee = cctpProtocolFee(amount, selectedRow.minimumFee);
    maxFee = bufferedCctpMaxFee(protocolFee);

    const provisionalOutput = outputAmountOf({
      bridgeType: req.bridgeType,
      amount,
      maxFee,
      mintFee: gasFees.mintFee,
    });
    const opp = opportunityCostQuote({ outputAmount: provisionalOutput, deliveryWindow });
    const fastFillEstimated = protocolFee + gasFees.mintFee + gasFees.baseFee + opp.targetTimeFee;

    // Apples-to-apples Circle-direct reference for *this* transfer's settings: the selected finality's
    // protocol fee, plus EITHER the forwarding (mint) fee when Relay Mint is on, OR — when it's off —
    // the destination gas the recipient must spend to mint the USDC themselves. Counting that gas keeps
    // the comparison honest: going direct without forwarding isn't free, you pay to claim.
    const forwarding = req.relayMint;
    const circleForwardFee = forwarding ? forwardFeeMed(feeRow(forwardFees, req.finality)) : 0n;
    const circleClaimGas = forwarding
      ? 0n
      : bufferedGasCostToken({ gasUnits: CCTP_DIRECT_SETTLE_GAS, gasPriceWei, ethUsd: ethUsdNow, decimals });
    // fast-fill runs any destination action (deposit/swap) atomically inside the fill — its gas is
    // already in `baseFee` (via fillGasUnits). Going direct, the recipient must run that action as a
    // SEPARATE transaction after the funds land, paying its gas plus a fresh tx's intrinsic overhead.
    const circleExecGas =
      callbackGasLimit > 0n
        ? bufferedGasCostToken({
            gasUnits: callbackGasLimit + STANDALONE_TX_BASE_GAS,
            gasPriceWei,
            ethUsd: ethUsdNow,
            decimals,
          })
        : 0n;
    const circleDirect = protocolFee + circleForwardFee + circleClaimGas + circleExecGas;
    const cctpDirectReceived = amount > circleDirect ? amount - circleDirect : 0n;

    comparison = {
      speed: req.finality === FINALITY_FINALIZED ? "slow" : "fast",
      forwarding,
      circleDirect: circleDirect.toString(),
      circleProtocolFee: protocolFee.toString(),
      circleForwardFee: circleForwardFee.toString(),
      circleClaimGas: circleClaimGas.toString(),
      circleExecGas: circleExecGas.toString(),
      cctpDirectReceived: cctpDirectReceived.toString(),
      fastFillEstimated: fastFillEstimated.toString(),
      savings: signedSavings(circleDirect, fastFillEstimated),
    };
  }

  const mintFee = req.bridgeType === BRIDGE_CCTP && req.relayMint ? gasFees.mintFee : 0n;
  const outputAmount = outputAmountOf({ bridgeType: req.bridgeType, amount, maxFee, mintFee });
  if (outputAmount <= 0n) throw new Error("fees exceed amount");

  const opp = opportunityCostQuote({ outputAmount, deliveryWindow });

  return {
    params: {
      maxFee: maxFee.toString(),
      mintFee: mintFee.toString(),
      baseFee: gasFees.baseFee.toString(),
      discountRate: opp.discountRate.toString(),
      deliveryWindow: deliveryWindow.toString(),
      outputAmount: outputAmount.toString(),
    },
    breakdown: {
      gasPriceWei: gasPriceWei.toString(),
      ethUsd: ethUsdNow.toString(),
      cctpMinimumFeeBps,
      cctpProtocolFee: req.bridgeType === BRIDGE_CCTP ? protocolFee.toString() : undefined,
      cctpMaxFeeBuffer: req.bridgeType === BRIDGE_CCTP ? (maxFee - protocolFee).toString() : undefined,
      fillGasUnits: gasFees.fillGasUnits.toString(),
      fillGasFee: gasFees.fillGasFee.toString(),
      directSettleGasUnits: gasFees.directSettleGasUnits.toString(),
      directSettleGasFee: gasFees.directSettleGasFee.toString(),
      executorGasUnits: gasFees.executorGasUnits.toString(),
      executorGasFee: gasFees.executorGasFee.toString(),
      secondsSaved: opp.secondsSaved.toString(),
      targetTimeFee: opp.targetTimeFee.toString(),
    },
    comparison,
  };
}

export async function assertGaslessPricing(req: GaslessBridgeRequest): Promise<void> {
  const amount = BigInt(req.inputAmount);
  const maxFee = BigInt(req.maxFee ?? "0");
  const mintFee = BigInt(req.mintFee ?? "0");
  const baseFee = BigInt(req.baseFee);
  const discountRate = BigInt(req.discountRate);
  const deliveryWindow = BigInt(req.deliveryWindow);
  const callbackGasLimit = BigInt(req.callbackGasLimit ?? "0");

  const required = await quoteBridgePricing({
    bridgeType: req.bridgeType,
    token: req.token,
    srcChainId: req.srcChainId,
    dstChainId: req.dstChainId,
    amount: req.inputAmount,
    finality: req.minFinalityThreshold ?? FINALITY_FAST,
    relayMint: req.bridgeType === BRIDGE_CCTP && mintFee > 0n,
    deliveryWindow: req.deliveryWindow.toString(),
    callbackGasLimit: callbackGasLimit.toString(),
  });

  if (maxFee < BigInt(required.params.maxFee)) throw new Error("underpriced CCTP maxFee");
  if (mintFee < BigInt(required.params.mintFee)) throw new Error("underpriced CCTP mintFee");
  if (baseFee < BigInt(required.params.baseFee)) throw new Error("underpriced baseFee");

  const actualOutput =
    req.bridgeType === BRIDGE_CCTP ? amount - maxFee - mintFee : BigInt(req.outputAmount);
  if (actualOutput <= 0n || BigInt(req.outputAmount) !== actualOutput) throw new Error("invalid output amount");
  const opp = opportunityCostQuote({ outputAmount: actualOutput, deliveryWindow });
  if (discountRate < opp.discountRate) throw new Error("underpriced discountRate");
}

export async function requiredFillFeeForOrder(args: {
  bridgeType: number;
  dstChainId: SupportedChainId;
  callbackGasLimit: bigint;
  mintFee: bigint;
  decimals: number;
}): Promise<bigint> {
  const [gasPriceWei, ethUsdNow] = await Promise.all([pub(args.dstChainId).getGasPrice(), ethUsd()]);
  const gasFees = gasBackedFees({
    bridgeType: args.bridgeType,
    relayMint: args.bridgeType === BRIDGE_CCTP && args.mintFee > 0n,
    callbackGasLimit: args.callbackGasLimit,
    gasPriceWei,
    ethUsd: ethUsdNow,
    decimals: args.decimals,
  });
  return gasFees.baseFee + MIN_FEE_TO_FILL;
}

export { OPPORTUNITY_APR_BPS };
