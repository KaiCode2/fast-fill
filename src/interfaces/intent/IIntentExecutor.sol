// SPDX-License-Identifier: MIT
pragma solidity ^0.8.26;

/// @title  IIntentExecutor
/// @notice Minimal, hand-written interface for the Rhinestone `IntentExecutor` ERC-7579 executor module
///         (rhinestonewtf/compact-utils). Only the two permissionless standalone entrypoints a fast-fill
///         hook needs are declared here, with structs laid out byte-identically to the originals so the
///         ABI encoding / function selectors match the deployed module exactly.
/// @dev    Hand-written to match this repo's convention (only `forge-std` + `solady` are vendored) and to
///         avoid pulling in the module's heavy dependency tree (the-compact, modulekit, …). The
///         `IntentExecutor` verifies the account's EIP-712 signature over the ops, consumes the nonce, and
///         runs the executions on the account — so it is safe for anyone (including a fast-fill hook) to
///         relay a user-signed `*ChainOps`.
interface IIntentExecutor {
    /// @notice A generic wrapper for an encoded ERC-7579 execution payload (see `SmartExecutionLib`).
    struct Operation {
        bytes data;
    }

    /// @notice Single-chain signed operations: the simplest standalone-intent shape.
    struct SingleChainOps {
        address account; // the smart account that signed and is executed on
        uint256 nonce; // replay protection (account-scoped)
        Operation ops; // the executions to run on the account
        bytes signature; // EIP-712 signature from `account` over the ops
    }

    /// @notice Multi-chain signed operations: chain-agnostic signature spanning several chains.
    struct MultiChainOps {
        address account;
        uint256 chainIndex; // index of the current chain in `otherChains`
        bytes32[] otherChains; // op hashes for the other chains in the multi-chain intent
        uint256 nonce;
        Operation ops;
        bytes signature;
    }

    /// @notice Validate the signature + nonce and execute `signedOps.ops` on `signedOps.account`.
    function executeSinglechainOps(SingleChainOps calldata signedOps) external;

    /// @notice Multi-chain variant of {executeSinglechainOps}.
    function executeMultichainOps(MultiChainOps calldata signedOps) external;
}
