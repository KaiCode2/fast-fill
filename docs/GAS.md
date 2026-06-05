# Gas benchmarks

How much gas the fast-fill adapters consume, and — the question that matters for a thin wrapper —
**how much fast-fill adds on top of using CCTP / the OFT directly.**

## TL;DR

| Operation | fast-fill adds (overhead) | Notes |
|---|---:|---|
| **CCTP initiate** (`initiateCCTP`, `mintFee = 0`) vs direct `depositForBurnWithHook` | **≈ 81k gas** | pull funds into the adapter, build + hash the order, assign the nonce, resolve the destination domain, `OrderCreated` |
| **CCTP settle** (`settle`) vs direct `receiveMessage` | **≈ 56k gas** | re-parse the authenticated message, 3 auth checks, write the order record, disburse to filler/recipient |
| **OFT initiate** (`initiateOFT`) vs direct `oft.send` | **≈ 62k gas** | same shape as CCTP initiate (resolves the destination eid) |
| **CCTP `fill`** | **≈ 59k gas (absolute)** | a fast-fill primitive — no bridge equivalent |
| **OFT settle** (`lzCompose`) | **≈ 40k gas (absolute)** | the compose callback *is* the settle on OFT; no bridge equivalent |

The direct-path overhead is small relative to a cross-chain transfer: a real CCTP burn through the adapter costs
**192,839 gas end-to-end** on an Ethereum mainnet fork (measured below), of which the bridge itself is
the large majority. fast-fill never adds a second message or escrow — it rides the bridge's own
authenticated channel. (Numbers are with the IR pipeline, `via_ir = true`, solc 0.8.35; a destination-execution
hook is opt-in and adds nothing when `hookData` is empty.)

Each adapter resolves its **local** chain config (USDC / CCTP messenger + transmitter, or the OFT
entrypoint + token + endpoint) **once at construction** and caches it in immutables — running the live
cross-check there too — so the per-op overheads above no longer pay a registry read or a cross-check on
the hot path; only **remote**-chain config (keyed by the order's dst/src chain) is read per op. Relayers
can additionally amortize the per-transaction base cost across many mints/fills via
`CctpExecutor.executeBatch` / `FastFillBase.fillBatch`.

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
the **production** [`FastFillConfig`](../src/config/FastFillConfig.sol) is a **pure constant** function
(no SLOADs). Local config is now resolved once at construction and cached as immutables — identical in
the mock bench and in production — so the only place the mock-vs-production gap still shows is the
**remote** config reads: the destination chain on initiate, the source chain on settle. Those make the
mock overhead deltas a modest **over-statement** of production cost (a few k on initiate); the fork
total below is the ground truth.

## Mock measurements (warm storage)

From `forge test --match-path test/gas/GasBench.t.sol -vv` (plain run — do **not** read absolute gas
off `--gas-report`, whose call tracing inflates every number):

```
--- CCTP ---
initiateCCTP (fast-fill)        :  99,400
depositForBurnWithHook (direct) :  18,625
  => initiate OVERHEAD          :  80,775
fill (fast-fill only)           :  59,489
settle unfilled (fast-fill)     :  83,898
receiveMessage (direct)         :  27,691
  => settle OVERHEAD            :  56,207
--- OFT ---
initiateOFT (fast-fill)         :  80,674
oft.send (direct)               :  18,509
  => initiate OVERHEAD          :  62,165
lzCompose settle (fast-fill)    :  39,929
```

> The absolute mock numbers are **illustrative only** — the real CCTP/LayerZero contracts cost far
> more than the trivial mocks, which is exactly why the bridge cost is subtracted out for the overhead.

## Fork measurement (real CCTP, Ethereum mainnet fork)

From `FOUNDRY_PROFILE=fork forge test --match-path test/fork/CctpForkE2E.t.sol -vv` (requires an RPC):

```
FORK real initiateCCTP gas (incl. real CCTP burn): 192,839
```

This is the full source-side cost a user pays to start a transfer through fast-fill, real CCTP burn
included. The fast-fill overhead (~62–81k) is a fraction of it; the rest is the bridge.

## Contract sizes

All contracts are well under the 24,576-byte EIP-170 limit (`forge build --sizes`):

| Contract | Runtime size | Margin |
|---|---:|---:|
| `CctpAdapter` | 18,927 B | 5,649 B |
| `CctpExecutor` | 7,494 B | 17,082 B |
| `OftAdapter` | 18,207 B | 6,369 B |
| `OftAdapterFactory` | 21,080 B | 3,496 B |
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
