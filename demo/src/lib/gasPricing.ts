import { BRIDGE_CCTP } from "./order";
import { DEFAULT_MAX_FEE_RATE, WAD } from "./pricing";

export const FAST_FILL_GAS = 59_495n;
export const CCTP_DIRECT_SETTLE_GAS = 83_897n;
export const CCTP_EXECUTOR_SETTLE_BUDGET_GAS = 350_000n;

export const GAS_BUFFER_BPS = 2_500n;
export const CCTP_MAX_FEE_BUFFER_BPS = 2_000n;
export const OPPORTUNITY_APR_BPS = 1_000n;
export const EXPECTED_FILL_DELAY_SECS = 15n;
export const SECONDS_PER_YEAR = 31_536_000n;

const BPS_SCALE_DECIMALS = 6;
const BPS_SCALE = 10n ** BigInt(BPS_SCALE_DECIMALS);
const BPS_DENOMINATOR = 10_000n;
const BUFFER_DENOMINATOR = 10_000n;
const WEI_PER_ETH = 10n ** 18n;

export function ceilDiv(n: bigint, d: bigint): bigint {
  if (d <= 0n) throw new Error("ceilDiv denominator must be positive");
  return n === 0n ? 0n : (n + d - 1n) / d;
}

export function pow10(decimals: number): bigint {
  if (!Number.isInteger(decimals) || decimals < 0) throw new Error(`invalid decimals ${decimals}`);
  return 10n ** BigInt(decimals);
}

export function parseDecimalToScaled(value: number | string, decimals = BPS_SCALE_DECIMALS): bigint {
  const raw =
    typeof value === "number"
      ? Number.isInteger(value)
        ? value.toString()
        : value.toFixed(decimals)
      : value.trim();
  if (!/^\d+(\.\d+)?$/.test(raw)) throw new Error(`invalid decimal value ${value}`);
  const [whole, frac = ""] = raw.split(".");
  const padded = (frac + "0".repeat(decimals)).slice(0, decimals);
  return BigInt(whole) * 10n ** BigInt(decimals) + BigInt(padded || "0");
}

/** CCTP protocol fee in token base units. `minimumFee` is Circle's bps value, e.g. 1.3. */
export function cctpProtocolFee(amount: bigint, minimumFeeBps: number | string): bigint {
  if (amount <= 0n) return 0n;
  const scaledBps = parseDecimalToScaled(minimumFeeBps);
  return ceilDiv(amount * scaledBps, BPS_DENOMINATOR * BPS_SCALE);
}

/** 20% buffered `maxFee` for CCTP source burns. */
export function bufferedCctpMaxFee(protocolFee: bigint): bigint {
  return ceilDiv(protocolFee * (BUFFER_DENOMINATOR + CCTP_MAX_FEE_BUFFER_BPS), BUFFER_DENOMINATOR);
}

/** Gas cost in USD-pegged token base units, with the 25% gas buffer included. */
export function bufferedGasCostToken(args: {
  gasUnits: bigint;
  gasPriceWei: bigint;
  ethUsd: bigint | number;
  decimals: number;
  gasBufferBps?: bigint;
}): bigint {
  const gasBufferBps = args.gasBufferBps ?? GAS_BUFFER_BPS;
  const ethUsd = typeof args.ethUsd === "bigint" ? args.ethUsd : BigInt(args.ethUsd);
  if (args.gasUnits <= 0n || args.gasPriceWei <= 0n || ethUsd <= 0n) return 0n;
  const numerator =
    args.gasUnits *
    args.gasPriceWei *
    ethUsd *
    pow10(args.decimals) *
    (BUFFER_DENOMINATOR + gasBufferBps);
  return ceilDiv(numerator, WEI_PER_ETH * BUFFER_DENOMINATOR);
}

export function fillGasUnits(callbackGasLimit: bigint): bigint {
  return FAST_FILL_GAS + callbackGasLimit;
}

export function cctpDirectFillGasUnits(callbackGasLimit: bigint): bigint {
  return FAST_FILL_GAS + callbackGasLimit + CCTP_DIRECT_SETTLE_GAS;
}

export function cctpExecutorGasUnits(callbackGasLimit: bigint): bigint {
  return CCTP_EXECUTOR_SETTLE_BUDGET_GAS + callbackGasLimit;
}

export function gasBackedFees(args: {
  bridgeType: number;
  relayMint: boolean;
  callbackGasLimit: bigint;
  gasPriceWei: bigint;
  ethUsd: bigint | number;
  decimals: number;
}) {
  const fillUnits = fillGasUnits(args.callbackGasLimit);
  const fillGasFee = bufferedGasCostToken({ ...args, gasUnits: fillUnits });

  const directSettleUnits = args.bridgeType === BRIDGE_CCTP && !args.relayMint ? CCTP_DIRECT_SETTLE_GAS : 0n;
  const directSettleGasFee =
    directSettleUnits > 0n ? bufferedGasCostToken({ ...args, gasUnits: directSettleUnits }) : 0n;

  const executorUnits = args.bridgeType === BRIDGE_CCTP && args.relayMint ? cctpExecutorGasUnits(args.callbackGasLimit) : 0n;
  const executorGasFee =
    executorUnits > 0n ? bufferedGasCostToken({ ...args, gasUnits: executorUnits }) : 0n;

  return {
    baseFee: fillGasFee + directSettleGasFee,
    mintFee: executorGasFee,
    fillGasUnits: fillUnits,
    fillGasFee,
    directSettleGasUnits: directSettleUnits,
    directSettleGasFee,
    executorGasUnits: executorUnits,
    executorGasFee,
  };
}

export function opportunityCostQuote(args: { outputAmount: bigint; deliveryWindow: bigint }) {
  const secondsSaved =
    args.deliveryWindow > EXPECTED_FILL_DELAY_SECS ? args.deliveryWindow - EXPECTED_FILL_DELAY_SECS : 0n;
  if (args.outputAmount <= 0n || secondsSaved === 0n) {
    return { secondsSaved, targetTimeFee: 0n, discountRate: 0n };
  }

  const rawTargetTimeFee = ceilDiv(
    args.outputAmount * OPPORTUNITY_APR_BPS * secondsSaved,
    BPS_DENOMINATOR * SECONDS_PER_YEAR,
  );
  const maxTimeFee = (args.outputAmount * DEFAULT_MAX_FEE_RATE) / WAD;
  const targetTimeFee = rawTargetTimeFee > maxTimeFee ? maxTimeFee : rawTargetTimeFee;
  let discountRate = targetTimeFee === 0n ? 0n : ceilDiv(targetTimeFee * WAD, args.outputAmount * secondsSaved);
  if (discountRate > DEFAULT_MAX_FEE_RATE) discountRate = DEFAULT_MAX_FEE_RATE;

  return { secondsSaved, targetTimeFee, discountRate };
}
