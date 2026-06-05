# Demo Fast-Fill Hooks

Hooks are destination-chain contracts that act on bridged funds the instant they arrive. When an order
carries `hookData` and `order.recipient` is a contract, the fast-fill adapter transfers the output token to
the recipient and, **in the same atomic frame**, calls
`IFastFillReceiver.onFastFill(orderId, token, amount, hookData)` on it
(`src/interfaces/IFastFillReceiver.sol`, `src/CallbackExecutor.sol`).

The load-bearing safety property is **revert-to-redirect**: if the callback reverts, the adapter rolls the
transfer back and re-routes the funds by the revert data — `RedirectFunds(dest)` → `dest`; anything else →
the recipient's pull-payment claim ledger. So a hook that fails its action can hand the user back the
original token, and a failed action degrades to a plain transfer — funds are never stuck.

This repo ships three demonstration hooks. **Uniswap and Aave are deployed and validated; the IntentExecutor
hook is implemented but deferred** (see [Status](#status--validation)).

## Shared design — `BaseFastFillHook`

`src/hooks/BaseFastFillHook.sol` is the abstract base all three extend. It standardizes the redirect:

```solidity
function onFastFill(bytes32 orderId, address token, uint256 amount, bytes calldata hookData) external {
    address user = _userOf(hookData);
    try this.execute(token, amount, user, hookData) { emit HookExecuted(orderId, user, token, amount); }
    catch { revert IFastFillReceiver.RedirectFunds(user); }   // any failure → user gets the original token
}
```

The action runs inside an external self-call (`execute`), so **any** inner failure — a reverting swap, a
frozen Aave reserve, a rejected signature — rolls back every partial side effect and is converted into the
clean `RedirectFunds(user)` sentinel (rather than a non-redirect revert that would land funds in the claim
ledger). Concrete hooks implement `_userOf` (decode the beneficiary) and `_doAction` (the work), and route
proceeds to `user` directly. Hooks are stateless per-chain singletons: `order.recipient` is the hook, the
final user + params travel in `order.hookData`, and the contracts hold no balances between calls.

## The hooks

### UniswapSwapHook — `src/hooks/UniswapSwapHook.sol`
On delivery of a stable (USDC/USDT), swaps it into a target token via Uniswap V3 `SwapRouter02`, output sent
straight to the user. A failed swap (slippage, no liquidity) redirects the original stable to the user.

- `hookData = abi.encode(address user, address tokenOut, uint24 poolFee, uint256 amountOutMinimum, uint160 sqrtPriceLimitX96)`
- Router is an immutable constructor arg (per-chain, from `Addresses.sol`).

### AaveDepositHook — `src/hooks/AaveDepositHook.sol`
On delivery, supplies the token to Aave V3 with aTokens minted directly to the user (`supply(..., onBehalfOf:
user, ...)`). A failed supply (frozen/unlisted reserve) redirects the original token to the user.

- `hookData = abi.encode(address user, uint16 referralCode)`
- Pool is an immutable constructor arg (per-chain, from `Addresses.sol`).

### IntentExecutorHook — `src/hooks/IntentExecutorHook.sol` (deferred)
On delivery, forwards the funds to the user's ERC-7579 smart account and relays a user-signed
`IntentExecutor` payload to the deployed Rhinestone module's permissionless entrypoint
(`executeSinglechainOps` / `executeMultichainOps`), which verifies the account's signature, consumes the
nonce, and runs the executions on the account. A failure (bad signature, used nonce, reverting ops)
redirects the original funds to the account.

- `hookData = abi.encode(uint8 variant, bytes signedOps)` — `variant` 0 = single-chain, 1 = multi-chain;
  `signedOps` is the abi-encoded `SingleChainOps` / `MultiChainOps`.
- Executor is an immutable constructor arg (per-chain `IntentExecutor`, from `Addresses.sol`).

How "sign over the executions **and** the claim" works with no origin-side change: the sponsored
`OrderIntent` Permit2 witness (`src/libraries/PermitLib.sol`) already binds `hookDataHash`,
`callbackGasLimit`, and `recipient`, so one origin authorization (the permit, or directly submitting the
order on the source chain) commits to the exact destination executions carried here.

## Deployed addresses

Base, Optimism, Arbitrum — deployer `0xA06Bf163BC51A457D99C6283e78897727c4fDdF2`, CREATE2-deterministic.
OP and ARB share an address per hook (same router/pool); Base differs. Tx hashes in
[`DEPLOYMENTS.md`](../DEPLOYMENTS.md#demo-hooks-destination-execution-callbacks).

| Hook | Base | Optimism · Arbitrum |
|---|---|---|
| `UniswapSwapHook` | `0xDeAF6072b2774a49688Fd09817Be9FBFbdE2835e` | `0x913FC613BE7a603Dc222Bce1997Ae28Fd7c48665` |
| `AaveDepositHook` | `0xBE30475CaEEd5003541DbAA8973bb01bA8433DC3` | `0xA0eCA1b76ff575B4031c510862f1024deFEEE321` |
| `IntentExecutorHook` | not deployed | not deployed |

## Using a hook from an order

Set three fields on the order (the demo builder `demo/src/lib/order.ts` already exposes them):

1. `recipient` = the hook address (bytes32) on the destination chain.
2. `callbackGasLimit` = gas for the action (≤ 5,000,000; e.g. ~400–500k for a swap/deposit). Priced into the fee.
3. `hookData` = the per-hook encoding above (the final user is always recoverable from it).

If the recipient is an EOA, or `hookData` is empty, the adapter simply delivers the funds with no callback.

## Status & validation

| Hook | Implemented | Adapter integration (real `CctpAdapter`, mocked protocol) | Real-protocol fork test | Deployed |
|---|:--:|:--:|:--:|:--:|
| UniswapSwapHook | ✅ | ✅ success + redirect | ✅ ETH/OP/ARB/Base, real Uniswap V3 | ✅ OP/ARB/Base |
| AaveDepositHook | ✅ | ✅ success + redirect | ✅ ETH/OP/ARB/Base, real Aave V3 | ✅ OP/ARB/Base |
| IntentExecutorHook | ✅ | ✅ success + redirect (mock executor) | ❌ none | ❌ deferred |

Tests: `test/integration/hooks/*` (mock-based, run in CI) and `test/fork/hooks/*` (env-gated, real
Uniswap/Aave on all four chains, asserting both the live action and the `RedirectFunds(user)` sentinel on a
forced failure).

### Why IntentExecutorHook is deferred

It is implemented and its outer call shape is verified (selectors `0x8f5c3522` / `0x34011077` and the struct
ABI match the module's source; the standalone entrypoints are permissionless). But it is **not validated
end-to-end against the real module**:

- It is tested only against `MockIntentExecutor` — a stand-in, so the test is close to circular.
- The mock's `Operation.data` is a simplified `(target, value, callData)`, **not** the real
  `SmartExecutionLib` / ERC-7579 `executeFromExecutor` encoding a real signed intent uses.
- No live `IntentExecutor` deployment is wired (`Addresses.intentExecutor(...)` returns `address(0)`, so
  `DeployHooks` skips it on every chain).
- The fund-flow assumption (hook deposits funds into the account, then the ops consume them) is plausible but
  unconfirmed against the real module + a real account with the module installed.

### Completion checklist

1. Obtain / confirm the deployed `IntentExecutor` address per chain and fill it into
   `INTENT_EXECUTOR_{ETHEREUM,OPTIMISM,ARBITRUM,BASE}` in `script/config/Addresses.sol`.
2. Cross-check the `Operation.data` encoding against `compact-utils` `SmartExecutionLib` (SigMode byte +
   ERC-7579 execution mode) and adjust the demo encoding/docs accordingly.
3. Add a fork test that builds a real EIP-712-signed `SingleChainOps` against an account with the module
   installed and runs it end-to-end (success + redirect), mirroring `test/fork/hooks/*`.
4. `forge script script/DeployHooks.s.sol --rpc-url $RPC --broadcast --private-key $KEY --slow` — once the
   address is set, the script deploys the hook (no code change needed); record the address in
   `DEPLOYMENTS.md` and `Addresses.sol`.
