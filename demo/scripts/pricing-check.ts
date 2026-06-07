import assert from "node:assert/strict";
import {
  bufferedCctpMaxFee,
  bufferedGasCostToken,
  CCTP_DIRECT_SETTLE_GAS,
  CCTP_EXECUTOR_SETTLE_BUDGET_GAS,
  cctpProtocolFee,
  FAST_FILL_GAS,
  gasBackedFees,
  opportunityCostQuote,
} from "../src/lib/gasPricing";
import { BRIDGE_CCTP, BRIDGE_OFT } from "../src/lib/order";
import { DEFAULT_MAX_FEE_RATE, feeOf } from "../src/lib/pricing";

// Circle bps decimal parsing + 20% CCTP maxFee buffer.
const protocolFee = cctpProtocolFee(10_000_000n, 1.3);
assert.equal(protocolFee, 1_300n);
assert.equal(bufferedCctpMaxFee(protocolFee), 1_560n);

// 25% gas buffer. 200k gas @ 0.001 gwei and $3000/ETH is 0.000600 USDC; buffered => 0.000750.
assert.equal(
  bufferedGasCostToken({ gasUnits: 200_000n, gasPriceWei: 1_000_000n, ethUsd: 3000n, decimals: 6 }),
  750n,
);

// 10% APR opportunity cost rounds up in token base units, then derives a rate that reaches it.
const opp = opportunityCostQuote({ outputAmount: 1_000_000n, deliveryWindow: 3_600n });
assert.equal(opp.secondsSaved, 3_585n);
assert.equal(opp.targetTimeFee, 12n);
assert.ok(
  feeOf({
    outputAmount: 1_000_000n,
    startTime: 0n,
    expectedDeliveryTime: 3_600n,
    fillTime: 15n,
    discountRate: opp.discountRate,
    maxFeeRate: DEFAULT_MAX_FEE_RATE,
    baseFee: 0n,
  }) >= opp.targetTimeFee,
);

// Fee split: routed CCTP separates executor gas into mintFee; direct CCTP includes settle gas in baseFee.
const routed = gasBackedFees({
  bridgeType: BRIDGE_CCTP,
  relayMint: true,
  callbackGasLimit: 100_000n,
  gasPriceWei: 1_000_000n,
  ethUsd: 3000n,
  decimals: 6,
});
assert.equal(routed.fillGasUnits, FAST_FILL_GAS + 100_000n);
assert.equal(routed.executorGasUnits, CCTP_EXECUTOR_SETTLE_BUDGET_GAS + 100_000n);
assert.equal(routed.baseFee, routed.fillGasFee);
assert.equal(routed.mintFee, routed.executorGasFee);

const direct = gasBackedFees({
  bridgeType: BRIDGE_CCTP,
  relayMint: false,
  callbackGasLimit: 100_000n,
  gasPriceWei: 1_000_000n,
  ethUsd: 3000n,
  decimals: 6,
});
assert.equal(direct.directSettleGasUnits, CCTP_DIRECT_SETTLE_GAS);
assert.equal(direct.baseFee, direct.fillGasFee + direct.directSettleGasFee);
assert.equal(direct.mintFee, 0n);

const oft = gasBackedFees({
  bridgeType: BRIDGE_OFT,
  relayMint: false,
  callbackGasLimit: 100_000n,
  gasPriceWei: 1_000_000n,
  ethUsd: 3000n,
  decimals: 6,
});
assert.equal(oft.baseFee, oft.fillGasFee);
assert.equal(oft.directSettleGasUnits, 0n);
assert.equal(oft.mintFee, 0n);

console.log("[pricing] OK — gas-backed pricing math checks passed.");
