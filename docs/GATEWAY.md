# Circle Gateway relayer filling — design (not yet implemented)

> **Status: design only.** This document specifies how a relayer would fund a CCTP fill from a Circle
> Gateway balance. No contract code or config for this exists in the repo yet — it is written up here
> so it is ready to build. The destination-execution feature (shipped) is independent of this.

## Motivation

Today a relayer must already hold USDC on the destination chain to `fill` an order. **Circle Gateway**
lets a relayer instead keep a single **unified USDC balance** (deposited once into Circle's
`GatewayWallet`) and **mint USDC on demand on any supported chain** in <1s, with Circle fronting the
liquidity. The goal: a relayer detects an `OrderCreated`, requests a Gateway mint, and **fills the
order with the freshly-minted USDC in one transaction** — no pre-positioned inventory per chain.

## How Circle Gateway works (relevant facts)

- Two contracts (same address across chains): **`GatewayWallet`** (deposit / withdraw / unified
  balance, on the source) and **`GatewayMinter`** (mint on the destination).
- The relayer signs an off-chain **EIP-712 burn intent** (a `TransferSpec`: source/destination
  domains, tokens, `sourceDepositor`, `destinationRecipient`, `value`, `salt`, optional
  `destinationCaller`, optional `hookData`) and POSTs it to Circle's Gateway API
  (`POST /v1/transfer`). The API returns an **operator-signed attestation**.
- Anyone holding the attestation calls **`GatewayMinter.gatewayMint(bytes attestationPayload, bytes
  signature)`** on the destination; it verifies Circle's operator signature, mints `value − fee` USDC
  to `destinationRecipient`, and marks the transfer spec used (replay guard). The mint is **instant**
  (Circle settles the source burn asynchronously).
- `gatewayMint` is **permissionless unless `destinationCaller` is set** in the burn intent (then only
  that address may submit it). Gateway reuses **CCTP domain IDs** (ETH 0, OP 2, ARB 3, BASE 6, …).
- Fees come out of the source balance: a small per-transfer fee (~0.005%) + a chain base fee, so
  `minted = value − fee`. The relayer chooses `value` so that `minted ≥ payout`.

## Recommended design — `fillViaGateway` (CCTP/USDC only)

Add to `CctpAdapter`:

```solidity
function fillViaGateway(Order calldata order, bytes calldata attestation, bytes calldata signature)
    external nonReentrant whenNotPaused returns (bytes32 orderId)
{
    address token; address recipient; uint256 payout; uint256 fee;
    (orderId, token, recipient, payout, fee) = _prepareFill(order, msg.sender);   // records filler = msg.sender
    IGatewayMinter(config.chainConfig(block.chainid).gatewayMinter).gatewayMint(attestation, signature);
    SafeTransferLib.safeTransferFrom(token, msg.sender, address(this), payout);    // pull the just-minted USDC
    _deliverWithHook(orderId, token, recipient, payout, order.hookData, order.callbackGasLimit);
    emit OrderFilled(orderId, msg.sender, payout, fee, uint40(block.timestamp));
}
```

The relayer's burn intent sets **`destinationRecipient = themselves`** and **`destinationCaller = the
adapter`**. They pre-approve the adapter for USDC once (or batch a `selfPermit` via `multicall`). The
flow reuses the existing `_prepareFill` + delivery (including destination executions) verbatim — the
only new step is the `gatewayMint` that tops the relayer up just-in-time.

### Why this is front-run-safe (the key property)

The attestation appears in the mempool when the relayer submits. A front-runner who copies it and calls
`fillViaGateway` themselves achieves nothing:

- `gatewayMint` mints to the **relayer** (the `destinationRecipient` is baked into the signed
  attestation, not chosen by the caller), **not** the front-runner.
- The adapter then pulls `payout` from **`msg.sender` = the front-runner**, who has no minted USDC and
  no allowance ⇒ `transferFrom` reverts ⇒ **the whole transaction reverts, rolling back the mint**.

So the relayer's Gateway balance is never consumed by anyone else, and no inventory is stranded. The
relayer should set `destinationCaller = the adapter` so the attestation can *only* be used via
`fillViaGateway` (this also stops a griefer from calling `gatewayMint` directly to involuntarily
mint to the relayer's wallet). No attestation parsing is required.

### Pieces to add

- A hand-written `^0.8` `IGatewayMinter` mirror (faithful to Circle's deployed ABI), e.g.:
  ```solidity
  interface IGatewayMinter { function gatewayMint(bytes calldata attestationPayload, bytes calldata signature) external; }
  ```
- A `gatewayMinter` address field on `ChainConfig` / `FastFillConfig` (per chain; `address(0)` ⇒ the
  chain has no Gateway ⇒ `fillViaGateway` reverts `UnsupportedChain`). Optionally cross-check
  `IGatewayMinter(...).domain() == config.cctpDomain` on use, mirroring the existing live cross-checks.
- Relayer tooling (off-chain): deposit → build & sign the burn intent → `POST /v1/transfer` → submit
  `fillViaGateway(order, attestation, signature)`.

### Alternative considered — mint to the adapter + parse the attestation

Mint to the adapter (`destinationRecipient = adapter`), then have the adapter **parse Circle's
attestation** to recover the burn's signer and record it as the filler (so a front-runner crediting
themselves is impossible), refunding any excess. This needs no relayer approval, but requires a
hand-written `GatewayMessageLib` mirroring Circle's exact attestation byte layout (confirmed against
the deployed contract). Heavier and more fragile than the recommended design — kept as a fallback.

## Testing note

Unlike the CCTP burn (which we drive for real on a fork) and Permit2 (real signatures), `gatewayMint`
requires **Circle's operator-signed attestation**, which cannot be forged in a fork test. Coverage will
therefore be: a **mock `GatewayMinter`** (mints a configured amount to the recipient encoded in a test
attestation) for the unit/integration lifecycle, plus an **interface-shape fork check** (call a view
like `domain()` on the real deployed `GatewayMinter` to confirm the address + ABI). A full
end-to-end run needs a testnet with the live Circle Gateway API.
