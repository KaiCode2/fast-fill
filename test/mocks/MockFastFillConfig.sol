// SPDX-License-Identifier: MIT
pragma solidity ^0.8.26;

import {IFastFillConfig, ChainConfig} from "../../src/interfaces/IFastFillConfig.sol";

/// @notice Settable stand-in for the production FastFillConfig, so tests can register arbitrary
///         (mock) chains/bridge contracts without baking real mainnet addresses.
contract MockFastFillConfig is IFastFillConfig {
    mapping(uint256 chainId => ChainConfig) internal _cfg;

    function set(uint256 chainId, ChainConfig memory c) external {
        _cfg[chainId] = c;
    }

    function chainConfig(uint256 chainId) external view returns (ChainConfig memory) {
        return _cfg[chainId];
    }
}
