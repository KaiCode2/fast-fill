// SPDX-License-Identifier: MIT
pragma solidity ^0.8.26;

/// @notice Per-chain transport configuration for one EVM chain (CCTP + the chain's LayerZero eid).
/// @dev `cctpDomain` 0 (Ethereum) is a valid domain, so presence is determined by `supported`,
///      never by a zero domain. `lzEid` is the chain's single LayerZero v2 endpoint id, shared by
///      every OFT on that chain. Per-OFT addresses live in the separate `oftConfig` lookup so the
///      registry scales to many OFTs without growing this struct (which is returned by value on
///      every resolve).
struct ChainConfig {
    bool supported; // this chain is known to the registry at all
    uint32 cctpDomain; // Circle CCTP domain
    uint32 lzEid; // LayerZero v2 endpoint id (per chain; shared across OFTs)
    address usdc; // canonical USDC (CCTP burn/mint token)
    address cctpTokenMessenger; // CCTP v2 TokenMessenger entrypoint (the MessageTransmitter is read from it)
}

/// @notice One OFT's deployment on one chain.
/// @dev `oft` is the entrypoint the adapter calls `send()` on (and reads `token()`/`endpoint()` from);
///      `token` is the ERC20 delivered/held on this chain. For a *native* mint/burn OFT the two are the
///      same address (`token() == address(oft)`); for an *adapter/lockbox* OFT (typically the home
///      chain, e.g. USD₮0/USDe/sUSDe/ENA/USDtb on Ethereum) they differ. Both `address(0)` ⇒ this OFT
///      is not available on this chain (the adapter reverts `UnsupportedChain`).
struct OftDeployment {
    address oft;
    address token;
}

/// @notice A chain registry the fast-fill adapters read for every cross-chain resolution. The
///         production implementation bakes every supported chain as immutable constants and is
///         CREATE2-deployed to one address on all chains, so adapters can take a single config
///         argument and still be deployed at a deterministic address everywhere.
///
///         OFTs are keyed by a small `oftId` (see `OftId`): USD₮0 = 0, USDe = 1, … . An OFT adapter
///         is constructed for one `oftId` and resolves its (oft, token) pair via `oftConfig` — so a
///         new OFT is onboarded by adding a registry row and deploying one more adapter, with no new
///         adapter code.
interface IFastFillConfig {
    /// @notice The transport configuration for `chainId`. `supported == false` if the chain is unknown.
    function chainConfig(uint256 chainId) external view returns (ChainConfig memory);

    /// @notice The deployment of OFT `oftId` on `chainId`. `oft == token == address(0)` if that OFT
    ///         is not deployed on that chain.
    function oftConfig(uint256 chainId, uint8 oftId) external view returns (OftDeployment memory);
}
