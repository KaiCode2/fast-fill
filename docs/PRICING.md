# Gas-backed pricing

fast-fill pricing has two layers:

1. The contracts enforce a simple signed fee curve: `baseFee + timeFee`.
2. The demo and relayers choose sane default signed values from factual inputs: gas benchmarks,
   live destination gas price, ETH/USD, and Circle CCTP fees.

This page documents the off-chain quote model used by the demo API and relayer profitability gates.
The on-chain order fields are unchanged:

| Field | Purpose |
|---|---|
| `maxFee` | CCTP source-side Circle protocol fee cap. CCTP may execute with `feeExecuted <= maxFee`. |
| `mintFee` | CCTP Relay Mint payment to whoever executes the destination mint through `CctpExecutor`. |
| `baseFee` | Flat fee paid to the optimistic filler when an order is filled. Used as the gas-backed floor. |
| `discountRate` | Per-second time-premium rate consumed by `PricingLib.quoteFill`. |
| `deliveryWindow` | Relative seconds from source initiation to expected bridge settlement. |

All token-denominated API fields are returned as decimal strings containing integer token base units
(`"1000000"` means `1.000000` for USDC / USDt0).

## Quote API

The demo form calls:

```http
POST /api/bridge/pricing
content-type: application/json
```

Request:

```json
{
  "bridgeType": 0,
  "token": "USDC",
  "srcChainId": 8453,
  "dstChainId": 42161,
  "amount": "1000000",
  "finality": 1000,
  "relayMint": true,
  "deliveryWindow": "60",
  "callbackGasLimit": "0"
}
```

Response:

```json
{
  "params": {
    "maxFee": "...",
    "mintFee": "...",
    "baseFee": "...",
    "discountRate": "...",
    "deliveryWindow": "...",
    "outputAmount": "..."
  },
  "breakdown": {
    "gasPriceWei": "...",
    "ethUsd": "...",
    "fillGasUnits": "...",
    "fillGasFee": "...",
    "directSettleGasUnits": "...",
    "directSettleGasFee": "...",
    "executorGasUnits": "...",
    "executorGasFee": "...",
    "secondsSaved": "...",
    "targetTimeFee": "..."
  },
  "comparison": {
    "circleFastForwarding": "...",
    "circleSlowForwarding": "...",
    "fastFillEstimated": "..."
  }
}
```

`comparison` is present only for CCTP routes.

## Inputs

The quote combines static gas budgets with live market data:

| Input | Source |
|---|---|
| Gas budgets | [`docs/GAS.md`](./GAS.md) |
| Destination gas price | Destination RPC `getGasPrice()` |
| ETH/USD | CoinGecko, with `RELAYER_ETH_PRICE_USD` fallback |
| CCTP protocol fees | Circle Iris `/v2/burn/USDC/fees/{sourceDomain}/{destDomain}` |
| CCTP forwarding fees | Same Circle endpoint with `?forward=true`, comparison only |

ETH/USD caching is controlled by:

| Env | Default |
|---|---:|
| `RELAYER_ETH_PRICE_URL` | `https://api.coingecko.com/api/v3/simple/price?ids=ethereum&vs_currencies=usd` |
| `RELAYER_ETH_PRICE_USD` | `3000` |
| `RELAYER_ETH_PRICE_TTL_SECS` | `14400` |

## Gas budgets

The pricing constants are:

| Constant | Gas |
|---|---:|
| `FAST_FILL_GAS` | `59,495` |
| `CCTP_DIRECT_SETTLE_GAS` | `83,897` |
| `CCTP_EXECUTOR_SETTLE_BUDGET_GAS` | `350,000` |
| `GAS_BUFFER_BPS` | `2,500` |

The gas buffer is 25%. It gives room for gas price movement between quote and execution and leaves a
small margin for relayers.

Gas is converted into USD-pegged token base units with:

```text
ceil(gasUnits * gasPriceWei * ethUsd * 10^tokenDecimals * 1.25 / 1e18)
```

For example, with a 6-decimal USD token:

```text
200,000 gas * 1,000,000 wei/gas * $3,000/ETH = 600 base units
buffered by 25% = 750 base units = $0.000750
```

This change intentionally scopes gas-to-token conversion to default USD-denominated routes (`USDC`
and `USDT0`). Non-USD OFTs need an asset price source before they should be enabled with this model.

## CCTP pricing

CCTP uses three fee buckets.

### `maxFee`

`maxFee` is the CCTP source-side protocol fee cap.

For fast CCTP (`finality = 1000`), the quote fetches Circle's `minimumFee` bps and computes:

```text
protocolFee = ceil(amount * minimumFeeBps / 10,000)
maxFee      = ceil(protocolFee * 1.20)
```

Circle can execute with `feeExecuted <= maxFee`. Any unspent CCTP fee is surplus that eventually
flows back through settlement accounting.

For finalized CCTP (`finality = 2000`), the quote uses Circle's finalized-fee row. On supported
routes this is normally zero.

### `mintFee`

`mintFee` is non-zero only when Relay Mint is enabled.

```text
mintFee = bufferedGasCost(350,000 + callbackGasLimit)
```

It pays whoever calls `CctpExecutor.execute(message, attestation)`. This replaces a hand-configured
flat mint fee with a route-specific gas quote.

### `baseFee`

`baseFee` is the optimistic filler gas floor.

Relay Mint on:

```text
baseFee = bufferedGasCost(59,495 + callbackGasLimit)
```

Relay Mint off:

```text
baseFee = bufferedGasCost(59,495 + callbackGasLimit + 83,897)
```

Relay Mint off means the filler must cover both the optimistic fill and the direct CCTP settlement
floor, so the direct settlement gas is included in `baseFee`.

The CCTP recipient output is:

```text
outputAmount = amount - maxFee - mintFee
```

## OFT pricing

OFT routes do not have CCTP `maxFee` or `mintFee`.

```text
maxFee       = 0
mintFee      = 0
baseFee      = bufferedGasCost(59,495 + callbackGasLimit)
outputAmount = amount
```

LayerZero's native messaging fee remains separate and is displayed separately in the UI. The user is
still paying LayerZero for delivery; fast-fill pricing only prices the optional speed-up.

## Opportunity cost

The time premium prices the relayer's capital opportunity cost. The default assumption is 10% APR
and a 15 second expected fill delay.

```text
secondsSaved  = max(deliveryWindow - 15, 0)
targetTimeFee = ceil(outputAmount * 1,000 * secondsSaved / (10,000 * 31,536,000))
```

Where:

| Term | Meaning |
|---|---|
| `1,000 / 10,000` | 10% APR in basis points |
| `31,536,000` | Seconds per year |
| `15` | Expected fill delay in seconds |

The quote then derives `discountRate` so the on-chain `PricingLib` fee curve reaches at least
`targetTimeFee` at the expected 15 second fill time:

```text
discountRate = ceil(targetTimeFee * 1e18 / (outputAmount * secondsSaved))
```

The derived rate is capped by the adapter's `maxFeeRate` behavior. This keeps the fee compatible with
the same on-chain curve used by all orders.

For small demo transfers this often rounds up to exactly one token base unit. That is expected:
10% APR for a few minutes or seconds is extremely cheap.

## CCTP comparison panel

The UI shows a CCTP-only comparison so users can compare fast-fill with using Circle directly.

| Line | Formula |
|---|---|
| Circle fast + forwarding | Unbuffered Circle fast protocol fee + Circle `forwardFee.med` |
| Circle slow + forwarding | Circle finalized forwarding fee, usually only `forwardFee.med` |
| fast-fill estimate | Expected Circle protocol fee + Relay Mint fee + base fill gas + opportunity cost |

The comparison is hidden for OFTs because LayerZero fees are always charged by the bridge; fast-fill
only changes the speed/relayer premium.

## Relayer enforcement

The quote is not only UI display. Relayers enforce it before spending capital.

The demo fallback relayer:

- Recomputes current pricing in `submitGasless`.
- Rejects signed gasless orders whose `maxFee`, `mintFee`, `baseFee`, or `discountRate` are below the
  current required quote.
- Recomputes the current gas-backed fill floor in `fillOrder`.
- Skips fills whose quoted `feeToFiller` is below the gas-backed floor plus `RELAYER_MIN_FEE`.

The Rust relayer:

- Uses the same gas conversion math beside `price.rs`.
- For routed CCTP fills, requires:

```text
feeToFiller >= fillGasFloor + RELAYER_MIN_FEE
```

- For direct CCTP fills, requires:

```text
feeToFiller >= fillGasFloor + directSettleGasFloor + RELAYER_MIN_FEE
```

- For OFT fills, requires:

```text
feeToFiller >= fillGasFloor + RELAYER_MIN_FEE
```

The existing `CctpExecutor.execute` gas-estimate profitability gate remains in place for mint
relaying; `mintFee` must still cover actual executor settlement profitability.

## Why this is deliberately conservative

The model is designed to avoid hidden subsidies:

- Users see signed order values before submission.
- Relayers can reject stale or underpriced quotes.
- Gas prices are buffered but still derived from live destination data.
- Circle protocol and forwarding comparisons come from Circle's API, not static assumptions.
- The contract fields remain stable, so no Solidity migration is required.

If new non-USD tokens are enabled, the missing piece is a token/USD price source. Until then, gas
floors should stay scoped to USD-denominated tokens.
