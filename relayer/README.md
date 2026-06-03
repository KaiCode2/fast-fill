# fast-fill relayer bot

An autonomous Rust relayer for the fast-fill demo. It watches `OrderCreated` events on every source
chain, reconstructs and verifies each order, fills the profitable ones it can cover, and — for CCTP
when the forwarding service is disabled — polls Circle's attestation API and relays the mint on-chain
so it gets reimbursed for the capital it fronted. OFT settlement is auto-delivered by LayerZero.

Built with [alloy](https://github.com/alloy-rs/alloy); contract bindings are generated with
`forge bind` (committed under `bindings/`).

## Layout

```
relayer/
├── bindings/             # `forge bind` alloy bindings (generated; committed)
└── bot/src/
    ├── main.rs           # runtime wiring: bootstrap → approvals → watchers → orchestrator
    ├── config.rs         # chain table, deployed addresses, env-driven settings
    ├── registry.rs       # on-chain bootstrap (FastFillConfig + OftAdapterFactory) → watch set + token map
    ├── sol.rs            # canonical Order/Execution aliases, IERC20, CCTP↔OFT order conversion
    ├── order.rs          # orderId = keccak256(abi.encode(order)); address⇆bytes32 helpers
    ├── watcher.rs        # per-chain getLogs poll (block cursor, ≤2000-block paging, reorg offset)
    ├── verify.rs         # reconstruct full Order from initiate calldata + authenticity gate
    ├── fill.rs           # idempotency → inventory → quote → policy gates → simulate → fill
    ├── cctp.rs           # Circle Iris v2 attestation poll + settle()
    ├── store.rs          # in-memory job store + lifecycle phases
    └── orchestrator.rs   # verify+fill on discovery; periodic tick drives attestation → settle
```

## How it works

1. **Watch** — one task per chain polls `eth_getLogs` for `OrderCreated` from the CCTP adapter and
   every deployed OFT adapter, only up to `head - SOURCE_CONFIRMATIONS` (reorg safety).
2. **Verify** — fetch the source tx, decode the `initiate*` calldata (unwrapping
   `multicall([selfPermit, initiate])`), read the source block timestamp, derive tokens from the
   registry, then **recompute `orderId` and require it to equal the emitted one** (the authenticity
   gate — fill a wrong order and you are never reimbursed).
3. **Fill** — skip if already filled (`getOrder`); check token policy + size cap; check inventory
   (`balanceOf`); quote the fee (`quoteFill`) against `RELAYER_MIN_FEE`; simulate then send `fill`.
   The per-chain lock serializes simulate→send→receipt so nonces never race.
4. **Settle** — CCTP: poll Circle Iris until `complete`, then `settle(message, attestation)` to mint
   and be reimbursed (only for orders we filled). OFT: just watch `getOrder` until LayerZero settles.

## Build & test

```bash
# (Re)generate bindings — run from the repo root after `forge build`:
forge bind --bindings-path relayer/bindings --crate-name fastfill-bindings \
  --select '^(CctpAdapter|OftAdapter|OftAdapterFactory|FastFillConfig)$' --overwrite

cd relayer
cargo build
cargo test                                   # offline unit tests (orderId framing, address casts)
cargo test -- --ignored golden_orderid_base_cctp   # golden gate vs a real mainnet order (needs Base RPC)
```

## Run

Copy `.env.example` to `.env` and fill it in (at minimum `ALCHEMY_API_KEY`; `RELAYER_PRIVATE_KEY` for
live mode). Then:

```bash
cargo run -- --dry-run     # detect + verify + quote + simulate; never sends txs
cargo run                  # LIVE: sends real fill/settle txs (funds at risk)
```

Live mode requires a hot wallet funded with the output tokens on the destination chains; the bot
pre-approves each adapter at startup. Guards: per-token enable list + size cap (`RELAYER_ENABLED_TOKENS`,
`RELAYER_MAX_*`, default USDC + USDT0 only), `RELAYER_MIN_FEE`, and `RELAYER_SRC_CONFIRMATIONS`.

> Use a consistent RPC provider (e.g. Alchemy). Free/load-balanced public RPCs rate-limit the
> bootstrap and can return inconsistent heads to `eth_getLogs`; the watcher tolerates this by logging
> and restarting, but a real key avoids it entirely.
