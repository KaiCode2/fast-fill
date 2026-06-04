# Live-test harness

Bash + [`cast`](https://book.getfoundry.sh/cast) scripts that exercise the relayer against **mainnet**
with real (small) CCTP orders. They drive the deployed contracts directly — no node/TS toolchain — and
read config from the repo-root `.env` (`ALCHEMY_API_KEY`, `DEMO_PRIVATE_KEY`).

> Real funds. Orders are ~0.1 USDC with low caps; the wallet needs a little USDC + ETH on each chain.
> The harness only **initiates** orders and observes; all fills/settles/executes are done by the
> relayer process under test.

## Scripts

| Script | Purpose |
|---|---|
| `lib.sh` | Shared helpers (env, chain/RPC/USDC table, `cast` wrappers, orderId/status parsing). Sourced by the rest. |
| `check-funding.sh` | Print the wallet's ETH / USDC / adapter-allowance on every chain. Read-only. |
| `initiate-cctp.sh SRC DST AMOUNT MAXFEE MINTFEE [RECIPIENT]` | Initiate one CCTP order (amounts in USDC base units, 6-dp). `MINTFEE>0` ⇒ executor-routed. Prints `ORDER_ID=` / `SRC_TX=`. |
| `watch-order.sh DST ORDER_ID [TIMEOUT] [POLL]` | Poll `getOrder` until the order Settles (status 2) or times out. |
| `live-test.sh` | Orchestrator: start the relayer (live, low caps), initiate the selected scenarios, watch them settle, and summarize PASS/FAIL with the relayer's own log lines per order. |

## Scenarios (`SCENARIOS` env, default `AB`)

- **A** — optimistic fill + **direct** settle (`mintFee=0`): exercises `fill()` then `CctpAdapter.settle`.
- **B** — optimistic fill + **executor** mint relay (`mintFee>0`, filled): exercises `fill()` then `CctpExecutor.execute` (relayer earns `mintFee` and recovers capital).
- **C** — **pure mint relay** of an order the relayer does *not* fill (runs with filling disabled): exercises the profitability-gated `CctpExecutor.execute` path — the permissionless drop-in for Circle's forwarding service. Uses a larger `mintFee` (`MINTFEE_C`, default 0.03 USDC) so it clears the gas gate.

## Usage

```bash
cd relayer/scripts
./check-funding.sh                 # preflight

./live-test.sh                     # Scenarios A + B (optimistic fill + executor, filled)
SCENARIOS=C ./live-test.sh         # Scenario C (unfilled pure mint relay)
SCENARIOS=ABC ./live-test.sh       # note: with filling enabled, C would be filled like B —
                                   #       run C on its own to test the unfilled path

# tunables: AMOUNT, MAXFEE, MINTFEE, MINTFEE_C, RELAYER_MAX_BASE_UNITS, WATCH_TIMEOUT
```

The relayer's stdout for each run is captured in `scripts/.live-test*-relayer.log` (git-ignored).
Each scenario passes when the relayer drives its order to `Settled` (status 2) on chain.
