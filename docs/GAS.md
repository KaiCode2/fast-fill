# Gas benchmarks

How much gas the fast-fill adapters consume, and — the question that matters for a thin wrapper —
**how much fast-fill adds on top of using CCTP / the OFT directly.**

## TL;DR

| Operation | fast-fill adds (overhead) | Notes |
|---|---:|---|
| **CCTP initiate** (`initiateCCTP`, `mintFee = 0`) vs direct `depositForBurnWithHook` | **≈ 77k gas** | pull funds into the adapter, build + hash the order, assign the nonce, one destination-config read, `OrderCreated` |
| **CCTP settle** (`settle`) vs direct `receiveMessage` | **≈ 56k gas** | re-parse the authenticated message, 3 auth checks, write the order record, disburse to filler/recipient |
| **OFT initiate** (`initiateOFT`) vs direct `oft.send` | **≈ 59k gas** | same shape as CCTP initiate (one destination chain + OFT read) |
| **CCTP `fill`** | **≈ 59k gas (absolute)** | a fast-fill primitive — no bridge equivalent |
| **OFT settle** (`lzCompose`) | **≈ 40k gas (absolute)** | the compose callback *is* the settle on OFT; no bridge equivalent |

The direct-path overhead is small relative to a cross-chain transfer: a real CCTP burn through the adapter costs
**~167,638 gas end-to-end** at a recent Ethereum mainnet block (measured below), of which the bridge itself is
the large majority. fast-fill never adds a second message or escrow — it rides the bridge's own
authenticated channel. (Numbers are with the IR pipeline, `via_ir = true`, solc 0.8.35; a destination-execution
hook is opt-in and adds nothing when `hookData` is empty.)

Gas-relevant design choices reflected above: each adapter resolves its **local** chain config once at
construction (cached as immutables) and reads its **destination**-chain config in a
**single** lookup per initiate (threaded into order-build + dispatch). `maxFeeRate` is downsized to `uint64`
and packed into one slot with `_nonceCounter` + `paused`, so on `fill` the pricing read is warm (the
`whenNotPaused` read already loaded the slot). Relayers can additionally amortize the per-transaction base
cost across many mints/fills via `CctpExecutor.executeBatch` / `FastFillBase.fillBatch`.

`mintFee > 0` CCTP orders route settlement through `CctpExecutor.execute` instead of `CctpAdapter.settle`.
That path adds one USDC transfer to the mint relayer plus the executor envelope decode and hook dispatch;
it is covered by `test/integration/CctpExecutor.t.sol` but is not yet included in this overhead bench.

## Methodology

Two layers, because each answers a different question:

1. **Mock overhead (faithful + reproducible).** The bench in [`test/gas/GasBench.t.sol`](../test/gas/GasBench.t.sol)
   measures each adapter op and the corresponding bare-bridge call against the **same in-EVM mock
   bridge**. Because the bridge call (`depositForBurnWithHook` / `oft.send` / `receiveMessage`) is
   byte-identical in the adapter path and the bare path, the mock's own cost **cancels in the
   subtraction** — so the *overhead* delta is faithful even though the mock's *absolute* numbers are
   not. Storage is warmed first so the deltas aren't skewed by one-time cold-SSTORE surcharges. **This is
   the number to track for what fast-fill itself costs.**

2. **Fork absolute (real, point-in-time).** The fork tests drive the **real** CCTP / USD₮0 contracts on a
   mainnet fork, so their totals are real-world. [`test/fork/CctpForkE2E.t.sol`](../test/fork/CctpForkE2E.t.sol)
   logs the real source-side gas. It re-forks the **latest** block, and the bridge portion (the majority of
   the total) moves with mainnet state, so the absolute **drifts across blocks** — earlier blocks measured
   ~193k–214k for the same operation. Treat it as a snapshot of the real-world total, not a precise
   before/after; the mock overhead above is the attributable, reproducible measure of fast-fill's share.

### Caveat on the overhead being an upper bound

The mock `FastFillConfig` is **storage-backed** (each `chainConfig` read is several SLOADs), whereas
the **production** [`FastFillConfig`](../src/config/FastFillConfig.sol) is a **pure constant** function
(no SLOADs). Local config is resolved once at construction and cached as immutables — identical in the
mock bench and in production — and the destination config is now a single read per initiate. The only
place the mock-vs-production gap still shows is that single remote read (mock SLOADs vs a pure return),
so the mock overhead deltas are a small **over-statement** of production; the fork total is the ground truth.

## Mock measurements (warm storage)

From `forge test --match-path test/gas/GasBench.t.sol -vv` (plain run — do **not** read absolute gas
off `--gas-report`, whose call tracing inflates every number):

```
--- CCTP ---
initiateCCTP (fast-fill)        :  95,729
depositForBurnWithHook (direct) :  18,625
  => initiate OVERHEAD          :  77,104
fill (fast-fill only)           :  59,495
settle unfilled (fast-fill)     :  83,897
receiveMessage (direct)         :  27,691
  => settle OVERHEAD            :  56,206
--- OFT ---
initiateOFT (fast-fill)         :  77,359
oft.send (direct)               :  18,509
  => initiate OVERHEAD          :  58,850
lzCompose settle (fast-fill)    :  39,929
```

> The absolute mock numbers are **illustrative only** — the real CCTP/LayerZero contracts cost far
> more than the trivial mocks, which is exactly why the bridge cost is subtracted out for the overhead.
> `maxFeeRate` packing saves an additional ~2.1k cold `SLOAD` on a real `fill` transaction; the bench
> pre-warms storage, so that one-time-per-tx saving does not appear in the warm `fill` figure above.

## Fork measurement (real CCTP, Ethereum mainnet fork)

From `FOUNDRY_PROFILE=fork forge test --match-path test/fork/CctpForkE2E.t.sol -vv` (requires an RPC):

```
FORK real initiateCCTP gas (incl. real CCTP burn): 167,638
```

The full source-side cost a user pays to start a transfer through fast-fill, real CCTP burn included.
Reproducible at a given block (three consecutive runs were identical) but drifts across blocks with the
bridge's mainnet state — see the methodology note above.

## Contract sizes

All contracts are well under the 24,576-byte EIP-170 limit (`forge build --sizes`):

| Contract | Runtime size | Margin |
|---|---:|---:|
| `CctpAdapter` | 18,919 B | 5,657 B |
| `CctpExecutor` | 7,494 B | 17,082 B |
| `OftAdapter` | 18,376 B | 6,200 B |
| `OftAdapterFactory` | 21,271 B | 3,305 B |
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
