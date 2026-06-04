# fast-fill relayer bot

An autonomous Rust relayer for the fast-fill demo. It watches `OrderCreated` events on every source
chain, reconstructs and verifies each order, and earns from **two independent roles**:

1. **Optimistic filling** — when it holds inventory and the fill is profitable, it calls `fill(order)`
   to pay the recipient instantly and is reimbursed at settlement (earning the fill fee).
2. **CCTP mint relaying** — for `mintFee > 0` orders that use the **CctpExecutor** (a permissionless,
   drop-in replacement for Circle's forwarding service), it polls Circle's attestation and calls
   `CctpExecutor.execute(message, attestation)`, which mints the USDC, pays the relayer `mintFee`, and
   forwards the rest to the adapter. This needs only gas (no inventory) and is pursued whenever
   `mintFee` covers the gas — even for orders the bot did not fill.

For `mintFee == 0` (direct) orders it filled, it settles via `CctpAdapter.settle`. OFT settlement is
auto-delivered by LayerZero. A routed order the bot fills earns *both* the fill fee and the `mintFee`
(and recovers its fronted capital) when it relays the mint.

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
    ├── cctp.rs           # Circle Iris v2 poll + settle() (direct) / execute() (mint relay) + profitability
    ├── store.rs          # in-memory job store + lifecycle phases
    └── orchestrator.rs   # verify+fill on discovery; tick drives settle (direct) / execute (mint relay)
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
   The per-chain lock serializes simulate→send→receipt so nonces never race. `outputAmount` reserves
   both the bridge fee and the `mintFee`: `inputAmount - maxFee - mintFee`.
4. **Settle / mint relay** — poll Circle Iris until `complete`, then:
   - routed (`mintFee > 0`): `CctpExecutor.execute(message, attestation)` — pursued for orders we
     filled (recover capital + earn `mintFee`) and for orders we did not fill when `mintFee` covers
     the estimated gas (`RELAYER_MIN_MINT_FEE`, `RELAYER_ETH_PRICE_USD`).
   - direct (`mintFee == 0`): `CctpAdapter.settle(message, attestation)`, only for orders we filled.
   - OFT: just watch `getOrder` until the LayerZero executor settles.

## Build & test

```bash
# (Re)generate bindings — run from the repo root after `forge build`:
forge bind --bindings-path relayer/bindings --crate-name fastfill-bindings \
  --select '^(CctpAdapter|CctpExecutor|OftAdapter|OftAdapterFactory|FastFillConfig)$' --overwrite

cd relayer
cargo build
cargo test                                   # offline unit tests (orderId framing, address casts)
cargo test -- --ignored verify::tests        # golden gates vs real mainnet orders (direct + routed)
```

## Run

Copy `.env.example` to `.env` and fill it in (at minimum `ALCHEMY_API_KEY`; `RELAYER_PRIVATE_KEY` for
live mode). Then:

```bash
cargo run -- --dry-run     # detect + verify + quote + simulate; logs fill/relay decisions, no txs
cargo run                  # LIVE: sends real fill / settle / execute txs (funds at risk)
```

Live mode requires a hot wallet funded with the output tokens on the destination chains for
*filling*; the bot pre-approves each adapter at startup. *Mint relaying* needs only gas (the executor
pays the bot in USDC). Guards: per-token fill enable list + size cap (`RELAYER_ENABLED_TOKENS`,
`RELAYER_MAX_*`, default USDC + USDT0 only), `RELAYER_MIN_FEE`, `RELAYER_SRC_CONFIRMATIONS`, and for
mint relay `RELAYER_MINT_RELAY` / `RELAYER_MIN_MINT_FEE` / `RELAYER_ETH_PRICE_USD`.

Deployed addresses (CREATE2-identical on Base / Optimism / Arbitrum; source `DEPLOYMENTS.md`):
`FastFillConfig 0xaec766…3DF5`, `CctpAdapter 0x9FA37f…bA75` (executor-enabled),
`CctpExecutor 0xAFc7bB…2a80`, `OftAdapterFactory 0x84Bb5d…bFcb`.

> Use a consistent RPC provider (e.g. Alchemy). Free/load-balanced public RPCs rate-limit the
> bootstrap and can return inconsistent heads to `eth_getLogs`; the watcher tolerates this by logging
> and restarting, but a real key avoids it entirely.
