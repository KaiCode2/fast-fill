# Gas benchmarks

How much gas the fast-fill adapters consume, and — the question that matters for a thin wrapper —
**how much fast-fill adds on top of using CCTP / the OFT directly.**

## TL;DR

| Operation | fast-fill adds (overhead) | Notes |
|---|---:|---|
| **CCTP initiate** (`initiateCCTP`, `mintFee = 0`) vs direct `depositForBurnWithHook` | **≈ 88k gas** | pull funds into the adapter, build + hash the order, assign the nonce, live domain cross-check, `OrderCreated` |
| **CCTP settle** (`settle`) vs direct `receiveMessage` | **≈ 60k gas** | re-parse the authenticated message, 3 auth checks, write the order record, disburse to filler/recipient |
| **OFT initiate** (`initiateOFT`) vs direct `oft.send` | **≈ 71k gas** | same shape as CCTP initiate (token + eid cross-check instead of domain) |
| **CCTP `fill`** | **≈ 61k gas (absolute)** | a fast-fill primitive — no bridge equivalent |
| **OFT settle** (`lzCompose`) | **≈ 44k gas (absolute)** | the compose callback *is* the settle on OFT; no bridge equivalent |

The direct-path overhead is small relative to a cross-chain transfer: a real CCTP burn through the adapter costs
**213,959 gas end-to-end** on an Ethereum mainnet fork (measured below), of which the bridge itself is
the large majority. fast-fill never adds a second message or escrow — it rides the bridge's own
authenticated channel. (Numbers are with the IR pipeline, `via_ir = true`; a destination-execution hook
is opt-in and adds nothing when `hookData` is empty.)

`mintFee > 0` CCTP orders route settlement through `CctpExecutor.execute` instead of `CctpAdapter.settle`.
That path adds one USDC transfer to the mint relayer plus the executor envelope decode and hook dispatch;
it is covered by `test/integration/CctpExecutor.t.sol` but is not yet included in this overhead bench.

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
initiateCCTP (fast-fill)        : 106,215
depositForBurnWithHook (direct) :  18,712
  => initiate OVERHEAD          :  87,503
fill (fast-fill only)           :  61,200
settle unfilled (fast-fill)     :  88,083
receiveMessage (direct)         :  27,691
  => settle OVERHEAD            :  60,392
--- OFT ---
initiateOFT (fast-fill)         :  89,769
oft.send (direct)               :  18,616
  => initiate OVERHEAD          :  71,153
lzCompose settle (fast-fill)    :  43,656
```

> The absolute mock numbers are **illustrative only** — the real CCTP/LayerZero contracts cost far
> more than the trivial mocks, which is exactly why the bridge cost is subtracted out for the overhead.

## Fork measurement (real CCTP, Ethereum mainnet fork)

From `FOUNDRY_PROFILE=fork forge test --match-path test/fork/CctpForkE2E.t.sol -vv` (requires an RPC):

```
FORK real initiateCCTP gas (incl. real CCTP burn): 213,959
```

This is the full source-side cost a user pays to start a transfer through fast-fill, real CCTP burn
included. The fast-fill overhead (~80–91k) is a fraction of it; the rest is the bridge.

## Contract sizes

Both adapters are well under the 24,576-byte EIP-170 limit (`forge build --sizes`):

| Contract | Runtime size | Margin |
|---|---:|---:|
| `CctpAdapter` | 18,452 B | 6,124 B |
| `CctpExecutor` | 6,278 B | 18,298 B |
| `OftAdapter` | 17,662 B | 6,914 B |
| `OftAdapterFactory` | 19,543 B | 5,033 B |
| `FastFillConfig` | 1,785 B | 22,791 B |

## Reproduce

```bash
# Faithful overhead deltas + illustrative absolutes (no RPC needed):
forge test --match-path test/gas/GasBench.t.sol -vv

# Per-function min/avg/max (numbers are tracing-inflated; use for relative comparison only):
forge test --match-path test/gas/GasBench.t.sol --gas-report

# Real source-side total against live CCTP (needs ETH_RPC_URL or ALCHEMY_API_KEY):
FOUNDRY_PROFILE=fork forge test --match-path test/fork/CctpForkE2E.t.sol -vv
```
