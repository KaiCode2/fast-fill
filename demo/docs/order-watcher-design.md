# Design: independent `OrderCreated` watcher (not yet implemented)

## Why

Today the relayer only learns about a new order when the **client POSTs** the source tx hash to
`POST /api/bridge/self` (or `/api/bridge/gasless`). If the browser errors before posting, or the
user closes the tab, the order is never filled. We want the API to **independently** discover
`OrderCreated` events — with **no webhooks and no Alchemy-specific features** (plain JSON-RPC only).

This is a *safety net* that complements, not replaces, the client-POST fast path: the POST stays the
low-latency path; the watcher catches anything it misses.

Viable here because the demo runs as a long-lived Node process (`next start`), so `setInterval`
survives (the relayer ticker in `src/lib/server/relayer.ts` already relies on this), and the adapters
are CREATE2-deterministic — **one address per bridgeType across every chain**.

## Shape — `src/lib/server/watcher.ts`

`ensureWatcher()` with a lazy single-start guard (mirrors `ensureTicker()`), one independent
`setInterval` per chain. Each tick, per chain:

1. `head = getBlockNumber()`; `toBlock = head − SOURCE_CONFIRMATIONS`. If `toBlock < cursor`, skip.
2. In `CHUNK_BLOCKS`-sized windows from `cursor + 1` to `toBlock`:
   ```ts
   const logs = await pub(chainId).getLogs({
     address: configuredAdapterAddresses(),        // [cctp, oft]; skip an unset adapter
     event: orderCreatedEvent,                      // parseAbiItem(...) — viem supports an address[] + single event
     fromBlock, toBlock: chunkEnd,
   });
   ```
3. For each log: `if (jobByTx(chainId, log.transactionHash)) continue;` else
   `registerJob(await reconstructAndVerify(chainId, log.transactionHash), false)`.
   - **Reuse `reconstructAndVerify` + `registerJob`** so the single authenticity gate (recompute
     `orderId`, assert it equals the emitted one, then `assertOrderAllowed`) is not duplicated.
   - Wrap each log in its own `try/catch` so one bad tx doesn't abort the batch.
4. Advance the in-memory cursor **per chunk** on success. On any chain-level error, log and **do not
   advance** the cursor (retry the same range next tick). Per-chain `scanning` re-entrancy guard.

## Idempotency (both POST and watcher see the same order)

Two dedup layers, both keyed by the deterministic `orderId` and both already in `store.ts`:

- `byTx` (`chainId:txHash → orderId`): the `self` route and the watcher both pre-check `jobByTx`, so
  the second observer skips before doing any RPC work.
- `registerJob` calls `getJob(orderId)` and returns the existing job if present, so even a true race
  past the `byTx` check produces no duplicate and no double-fill.

Net effect: if the POST wins, the watcher is a no-op; if the browser bug fires and never POSTs, the
watcher discovers the order within ≤ one poll interval and the order fills normally.

## Startup / lifecycle

- Add `demo/instrumentation.ts` exporting `register()`, guarded by
  `process.env.NEXT_RUNTIME === "nodejs"`, which dynamic-imports the watcher and calls
  `ensureWatcher()`. This runs once at server startup, even if no client ever POSTs.
- Also lazy-start from the `orders/[orderId]` GET and the bridge routes (belt-and-suspenders).
- The module-level `started` guard tolerates dev HMR / double-invoke (same guarantee `ensureTicker`
  relies on).

## Defaults (env-overridable, following `env.ts` conventions)

| Constant | Default | Env var |
|---|---|---|
| poll interval | 5000 ms | `WATCHER_POLL_MS` |
| chunk size | 2000 blocks | `WATCHER_CHUNK_BLOCKS` |
| boot lookback | 600 blocks | `WATCHER_BOOT_LOOKBACK` |
| confirmations | reuse `SOURCE_CONFIRMATIONS` (1) | `RELAYER_SRC_CONFIRMATIONS` |

Seed the cursor at `head − BOOT_LOOKBACK` so a brief restart re-discovers recent orders (idempotency
makes re-scanning free). Idle RPC cost ≈ 1–2 calls/chain/tick (~1.2 calls/sec across 3 chains); boot
is a one-time chunked sweep.

## Honest limitations

- **In-memory cursor lost on redeploy** — orders older than the boot lookback that were never POSTed
  aren't recovered after a restart (same posture as the in-memory job store; a production version
  would persist cursor + jobs).
- **Latency** — adds ≈ poll interval + confirmations vs the instant POST, so keep both paths.
- **`getLogs` on default public RPCs** — range caps, rate limits, and occasional inconsistency vary;
  `CHUNK_BLOCKS` + retry-without-advance mitigate, and a real RPC key materially helps — **without**
  depending on any Alchemy-only feature (only plain `eth_getLogs`).

## Reuses (unchanged)

`reconstructAndVerify` (`verify.ts`), `registerJob` (`relayer.ts`), `jobByTx` (`store.ts`),
`pub` (`clients.ts`), `adapterAddressServer` + `SOURCE_CONFIRMATIONS` (`env.ts`),
`SUPPORTED_CHAIN_IDS` / `SupportedChainId` (`chains.ts`).
