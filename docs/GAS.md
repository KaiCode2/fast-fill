# Gas benchmarks

How much gas the fast-fill adapters consume, and — the question that matters for a thin wrapper —
**how much fast-fill adds on top of using CCTP / the OFT directly.**

## TL;DR

| Operation | fast-fill adds (overhead) | Notes |
|---|---:|---|
| **CCTP initiate** (`initiateCCTP`) vs direct `depositForBurnWithHook` | **≈ 91k gas** | pull funds into the adapter, build + hash the order, assign the nonce, live domain cross-check, `OrderCreated` |
| **CCTP settle** (`settle`) vs direct `receiveMessage` | **≈ 65k gas** | re-parse the authenticated message, 3 auth checks, write the order record, disburse to filler/recipient |
| **OFT initiate** (`initiateOFT`) vs direct `oft.send` | **≈ 72k gas** | same shape as CCTP initiate (token + eid cross-check instead of domain) |
| **CCTP `fill`** | **≈ 62k gas (absolute)** | a fast-fill primitive — no bridge equivalent |
| **OFT settle** (`lzCompose`) | **≈ 47k gas (absolute)** | the compose callback *is* the settle on OFT; no bridge equivalent |

The overhead is small relative to a cross-chain transfer: a real CCTP burn through the adapter costs
**212,517 gas end-to-end** on an Ethereum mainnet fork (measured below), of which the bridge itself is
the large majority. fast-fill never adds a second message or escrow — it rides the bridge's own
authenticated channel.

## Methodology

Two layers, because each answers a different question:

1. **Mock overhead (faithful).** The bench in [`test/gas/GasBench.t.sol`](../test/gas/GasBench.t.sol)
   measures each adapter op and the corresponding bare-bridge call against the **same in-EVM mock
   bridge**. Because the bridge call (`depositForBurnWithHook` / `oft.send` / `receiveMessage`) is
   byte-identical in the adapter path and the bare path, the mock's own cost **cancels in the
   subtraction** — so the *overhead* delta is faithful even though the mock's *absolute* numbers are
   not. Storage is warmed first so the deltas aren't skewed by one-time cold-SSTORE surcharges.

2. **Fork absolute (real).** The fork tests drive the **real** CCTP / USD₮0 contracts on a mainnet
   fork, so their totals are real-world. [`test/fork/CctpForkE2E.t.sol`](../test/fork/CctpForkE2E.t.sol)
   logs the real source-side gas.

### Caveat on the overhead being an upper bound

The mock `FastFillConfig` is **storage-backed** (each `chainConfig` read is several SLOADs), whereas
the **production** [`FastFillConfig`](../src/config/FastFillConfig.sol) is a **pure constant**
function (no SLOADs). The adapter reads the config several times per op, so the overhead deltas above
**over-state** the production cost by roughly the difference between mock SLOADs and constant returns
(order of ~5–15k on initiate). Real production overhead is therefore somewhat lower than the mock
delta; the fork total below is the ground truth.

## Mock measurements (warm storage)

From `forge test --match-path test/gas/GasBench.t.sol -vv` (plain run — do **not** read absolute gas
off `--gas-report`, whose call tracing inflates every number):

```
--- CCTP ---
initiateCCTP (fast-fill)        : 108,503
depositForBurnWithHook (direct) :  17,558
  => initiate OVERHEAD          :  90,945
fill (fast-fill only)           :  61,610
settle unfilled (fast-fill)     :  95,211
receiveMessage (direct)         :  30,660
  => settle OVERHEAD            :  64,551
--- OFT ---
initiateOFT (fast-fill)         :  89,305
oft.send (direct)               :  17,132
  => initiate OVERHEAD          :  72,173
lzCompose settle (fast-fill)    :  46,575
```

> The absolute mock numbers are **illustrative only** — the real CCTP/LayerZero contracts cost far
> more than the trivial mocks, which is exactly why the bridge cost is subtracted out for the overhead.

## Fork measurement (real CCTP, Ethereum mainnet fork)

From `FOUNDRY_PROFILE=fork forge test --match-path test/fork/CctpForkE2E.t.sol -vv` (requires an RPC):

```
FORK real initiateCCTP gas (incl. real CCTP burn): 212,517
```

This is the full source-side cost a user pays to start a transfer through fast-fill, real CCTP burn
included. The fast-fill overhead (~80–91k) is a fraction of it; the rest is the bridge.

## Contract sizes

Both adapters are well under the 24,576-byte EIP-170 limit (`forge build --sizes`):

| Contract | Runtime size | Margin |
|---|---:|---:|
| `CctpAdapter` | 16,022 B | 8,554 B |
| `OftAdapter` | ~16,908 B | ~7,668 B |
| `FastFillConfig` | 988 B | 23,588 B |

## Reproduce

```bash
# Faithful overhead deltas + illustrative absolutes (no RPC needed):
forge test --match-path test/gas/GasBench.t.sol -vv

# Per-function min/avg/max (numbers are tracing-inflated; use for relative comparison only):
forge test --match-path test/gas/GasBench.t.sol --gas-report

# Real source-side total against live CCTP (needs ETH_RPC_URL or ALCHEMY_API_KEY):
FOUNDRY_PROFILE=fork forge test --match-path test/fork/CctpForkE2E.t.sol -vv
```
