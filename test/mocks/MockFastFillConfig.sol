// SPDX-License-Identifier: MIT
pragma solidity ^0.8.26;

import {IFastFillConfig, ChainConfig, OftDeployment} from "../../src/interfaces/IFastFillConfig.sol";

/// @notice Settable stand-in for the production FastFillConfig, so tests can register arbitrary
///         (mock) chains/bridge contracts and OFT deployments without baking real mainnet addresses.
contract MockFastFillConfig is IFastFillConfig {
    mapping(uint256 chainId => ChainConfig) internal _cfg;
    mapping(uint256 chainId => mapping(uint8 oftId => OftDeployment)) internal _oft;

    function set(uint256 chainId, ChainConfig memory c) external {
        _cfg[chainId] = c;
    }

    function setOft(uint256 chainId, uint8 oftId, OftDeployment memory d) external {
        _oft[chainId][oftId] = d;
    }

    function chainConfig(uint256 chainId) external view returns (ChainConfig memory) {
        return _cfg[chainId];
    }

    function oftConfig(uint256 chainId, uint8 oftId) external view returns (OftDeployment memory) {
        return _oft[chainId][oftId];
    }
}
