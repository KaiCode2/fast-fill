// SPDX-License-Identifier: MIT
pragma solidity ^0.8.26;

import {Script, console2} from "forge-std/Script.sol";
import {FastFillConfig} from "../src/config/FastFillConfig.sol";

/// @notice CREATE2-deploy the immutable chain registry. Because its bytecode is identical on every
///         chain, the same salt + deployer yields the SAME address everywhere — which is what lets
///         the adapters take a single config argument and themselves be deterministic.
/// @dev Deploy this from the same EOA with the same salt on every chain.
contract DeployFastFillConfig is Script {
    bytes32 internal constant SALT = keccak256("fast-fill.config.v1");

    function run() external returns (FastFillConfig config) {
        vm.startBroadcast();
        config = new FastFillConfig{salt: SALT}();
        vm.stopBroadcast();
        console2.log("FastFillConfig:", address(config));
    }
}
