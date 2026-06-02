// SPDX-License-Identifier: MIT
pragma solidity ^0.8.26;

import {Test} from "forge-std/Test.sol";

/// @notice Shared RPC resolution for fork tests. Prefers an explicit *_RPC_URL env var; otherwise
///         builds an Alchemy URL from ALCHEMY_API_KEY. If neither is set, fork tests self-skip.
abstract contract ForkBase is Test {
    function _ethRpcUrl() internal view returns (string memory) {
        return _rpcUrl("ETH_RPC_URL", "eth-mainnet");
    }

    function _opRpcUrl() internal view returns (string memory) {
        return _rpcUrl("OP_RPC_URL", "opt-mainnet");
    }

    function _arbRpcUrl() internal view returns (string memory) {
        return _rpcUrl("ARB_RPC_URL", "arb-mainnet");
    }

    /// @dev Resolve an RPC: explicit env var wins; else build an Alchemy URL for `alchemyNetwork`.
    function _rpcUrl(string memory envVar, string memory alchemyNetwork) internal view returns (string memory) {
        string memory url = vm.envOr(envVar, string(""));
        if (bytes(url).length != 0) return url;
        string memory key = vm.envOr("ALCHEMY_API_KEY", string(""));
        if (bytes(key).length != 0) {
            return string.concat("https://", alchemyNetwork, ".g.alchemy.com/v2/", key);
        }
        return "";
    }

    /// @dev Selects an Ethereum mainnet fork, or marks the test skipped and returns false.
    function _forkMainnetOrSkip() internal returns (bool) {
        return _forkOrSkip(_ethRpcUrl());
    }

    /// @dev Selects a fork at `url`, or marks the test skipped and returns false when `url` is empty.
    function _forkOrSkip(string memory url) internal returns (bool) {
        if (bytes(url).length == 0) {
            vm.skip(true);
            return false;
        }
        vm.createSelectFork(url);
        return true;
    }
}
