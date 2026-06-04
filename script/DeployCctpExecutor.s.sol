// SPDX-License-Identifier: MIT
pragma solidity ^0.8.26;

import {Script, console2} from "forge-std/Script.sol";
import {CctpExecutor} from "../src/CctpExecutor.sol";

/// @notice CREATE2-deploy the canonical CctpExecutor against the deterministic FastFillConfig registry.
/// @dev Env: CONFIG (the FastFillConfig address — identical on every chain), optional OWNER. Deploy
///      from the same EOA with identical args (same CONFIG, OWNER) for a deterministic address across
///      chains. Deploy this BEFORE the CctpAdapter and pass the resulting address as EXECUTOR to
///      DeployCctpAdapter. No post-deploy wiring is needed — the executor resolves everything from
///      CONFIG at call time and is bridge-agnostic.
contract DeployCctpExecutor is Script {
    bytes32 internal constant SALT = keccak256("fast-fill.cctp-executor.v1");

    function run() external returns (CctpExecutor executor) {
        address config = vm.envAddress("CONFIG");
        address owner = vm.envOr("OWNER", msg.sender);

        vm.startBroadcast();
        executor = new CctpExecutor{salt: SALT}(config, owner);
        vm.stopBroadcast();

        console2.log("CctpExecutor:", address(executor));
        console2.log("config:", config);
        console2.log("owner:", owner);
    }
}
