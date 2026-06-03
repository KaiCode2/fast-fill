// SPDX-License-Identifier: MIT
pragma solidity ^0.8.26;

/// @title  OftId
/// @notice Stable numeric identifiers for each OFT the fast-fill protocol supports. An `OftAdapter`
///         is deployed once per id (via `OftAdapterFactory`), and the id keys the `(oft, token)`
///         lookup in `IFastFillConfig.oftConfig`. Ids are append-only: never renumber an existing
///         OFT — the id is baked into a deployed adapter (its immutable + its CREATE2 salt), so a
///         renumber would silently repoint a live, fund-holding adapter at a different token.
///
///         The id is NOT carried in the `Order` (an order's `bridgeType` is just `BRIDGE_OFT`); it is
///         a deploy-time property that isolates each OFT's adapter to its own address and pool.
library OftId {
    uint8 internal constant USDT0 = 0; // USD₮0 (Tether's omnichain USDT)
    uint8 internal constant USDE = 1; // Ethena USDe
    uint8 internal constant SUSDE = 2; // Ethena Staked USDe (sUSDe)
    uint8 internal constant ENA = 3; // Ethena governance token
    uint8 internal constant USDTB = 4; // Ethena USDtb (BUIDL-backed)
    // Append new OFTs here with the next free id. Reserved (researched, not yet enabled):
    //   PYUSD — limited chain coverage (ETH + ARB native; OP is the distinct PYUSD0 asset; no Base).
    //   WBTC  — BitGo OFT on ETH/OP/Base (no Arbitrum); LZ transfers were paused ~2026-04, re-verify.
}
