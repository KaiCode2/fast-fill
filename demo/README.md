<p align="center">
  <img src="../assets/fastfill-logo.png" alt="Fast Fill" width="420">
</p>

# fast-fill demo

A small Next.js app that showcases the [fast-fill](../README.md) optimistic-fill protocol on **real
mainnet** (Arbitrum, Optimism, Base): connect a wallet, see USDC / USD₮0 balances, start a transfer
tuning the fees / fast-vs-slow / backend CCTP auto-settle, optionally **gasless** (EIP-2612 or
Permit2), and watch a backend relayer fill on the destination in seconds and finalize settlement.

The checked-in demo UI keeps CCTP on the direct adapter settlement path with `mintFee = 0`. The
executor-routed path (`mintFee > 0`, settled through `CctpExecutor.execute`) is deployed and
live-smoke-tested, but the UI still needs an explicit control before users can opt into it.

> ⚠️ **Real money, unaudited prototype.** Defaults cap a transfer at a couple of dollars. The backend
> holds a hot relayer key. Use tiny amounts.

## What it demonstrates

- **Balances** for USDC (Arb/Op/Base) and USD₮0 (Arb/Op) across chains.
- **Three submit modes** for the same transfer:
  - **Standard** — approve + `initiateCCTP` / `initiateOFT` (you pay gas).
  - **EIP-2612 single-tx** — `multicall([selfPermit, initiate])` (one tx, no separate approve).
  - **Permit2 gasless** — you *sign* an `OrderIntent`; the relayer submits `initiate*For` and pays gas.
- **Tunable params:** flat `baseFee`, time-premium `discountRate` + `deliveryWindow`, CCTP `maxFee`,
  **fast vs finalized** (`minFinalityThreshold`), and a **CCTP auto-settle** toggle driven by the
  backend relayer.
- **Backend relayer** that authenticates each order (`keccak256(abi.encode(order))` must match the
  adapter's `OrderCreated`), **fills** on the destination, and for CCTP polls Circle's attestation and
  **settles**; OFT settlement auto-delivers via the LayerZero executor.
- A live **Created → Filled → Settled** timeline per transfer.

## Architecture

```
demo/
  src/app/                 UI (page, providers) + API routes (the relayer)
    api/bridge/self        register a self-submitted bridge → verify → fill
    api/bridge/gasless     submit a signed Permit2 intent → initiate*For → fill
    api/orders/[id]        lifecycle status
    api/settle/[id]        manual settle / forwarding kick
    api/health             relayer balances / allowances
    api/rpc/[chainId]      read-only JSON-RPC proxy (keeps the Alchemy key server-side)
  src/lib/                 shared codecs (order id, pricing, Permit2 witness, LZ options) + ABIs
  src/lib/server/          server-only relayer (key, clients, verify, fill, cctp, orchestrator)
  src/components, hooks/   wallet, balances, bridge form, timeline
  scripts/                 gen-abis, setup-approvals, parity-check
```

The Permit2 witness encoding is validated against the contract's literal type strings by
`scripts/parity-check.ts` (`pnpm test:parity`) — run it before trusting the gasless path.

## Setup

**1. Build the contracts** (generates the ABIs the app imports):

```bash
cd ..            # repo root
forge build
```

**2. Contracts are already deployed** on Base/Optimism/Arbitrum and **baked into the app** (see
[`../DEPLOYMENTS.md`](../DEPLOYMENTS.md) — `FastFillConfig`, the executor-enabled `CctpAdapter`,
`CctpExecutor`, and `OftAdapter`, all CREATE2-deterministic → one address everywhere). The historical
pre-executor `CctpAdapter` is also listed there for the original live demo record.

**3. Configure env:**

```bash
cd demo
cp .env.example .env.local
# fill in ALCHEMY_API_KEY and RELAYER_PRIVATE_KEY (addresses default to ../DEPLOYMENTS.md)
```

**4. Fund + approve the relayer** (ETH + USDC on Arb/Op/Base, USD₮0 on Arb/Op):

```bash
pnpm install
pnpm setup:approvals
```

**5. Run:**

```bash
pnpm dev        # http://localhost:3000   (predev regenerates ABIs from ../out)
```

`GET /api/health` shows the relayer's per-chain balances and allowances.

## Notes & limits

- **USD₮0 is 6 decimals** (verified on the OP fork), same as USDC.
- **Destination executions** (`onFastFill` callbacks) are supported by the contracts; this demo sends
  funds-only (empty `hookData`, `callbackGasLimit` 0) — the codec/witness carry the fields so it's a
  one-line change to enable.
- **CCTP Executor routing** is deployed and live-smoke-tested, but the UI sets `mintFee = 0`, so
  transfers initiated from the checked-in form are still settled directly through `CctpAdapter.settle`.
- **USD₮0 destination liquidity** must be seeded before gasless OFT fills work (or run one baseline
  transfer first, like the CCTP demo).
- The relayer's lifecycle store is **in-memory** — it resets on redeploy.
- Permit2 needs a one-time ERC20 approval of the canonical Permit2 (the UI prompts for it).
- Scripts: `pnpm gen:abis`, `pnpm setup:approvals`, `pnpm test:parity`, `pnpm typecheck`, `pnpm build`.
