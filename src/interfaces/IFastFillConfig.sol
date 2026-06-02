// SPDX-License-Identifier: MIT
pragma solidity ^0.8.26;

/// @notice Per-chain bridge configuration for one EVM chain.
/// @dev `cctpDomain` 0 (Ethereum) is a valid domain, so presence is determined by `supported`,
///      never by a zero domain. A field left at address(0)/eid 0 means "this bridge is not
///      available on this chain" (e.g. a chain with USDC/CCTP but no USD₮0 leaves the usdt0* zeroed).
struct ChainConfig {
    bool supported; // this chain is known to the registry at all
    uint32 cctpDomain; // Circle CCTP domain
    uint32 lzEid; // LayerZero v2 endpoint id
    address usdc; // canonical USDC (CCTP burn/mint token)
    address cctpTokenMessenger; // CCTP v2 TokenMessenger entrypoint (the MessageTransmitter is read from it)
    address usdt0Oft; // USD₮0 OFT entrypoint (the endpoint + token + eid are read from it)
    address usdt0Token; // USD₮0 ERC20 delivered on this chain
}

/// @notice A chain registry the fast-fill adapters read for every cross-chain resolution. The
///         production implementation bakes every supported chain as immutable constants and is
///         CREATE2-deployed to one address on all chains, so adapters can take a single config
///         argument and still be deployed at a deterministic address everywhere.
interface IFastFillConfig {
    /// @notice The configuration for `chainId`. `supported == false` if the chain is unknown.
    function chainConfig(uint256 chainId) external view returns (ChainConfig memory);
}
