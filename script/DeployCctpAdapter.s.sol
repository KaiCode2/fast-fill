// SPDX-License-Identifier: MIT
pragma solidity ^0.8.26;

import {Script, console2} from "forge-std/Script.sol";
import {CctpAdapter} from "../src/adapters/CctpAdapter.sol";

/// @notice CREATE2-deploy a CctpAdapter against the deterministic FastFillConfig registry.
/// @dev Env: CONFIG (the FastFillConfig address — identical on every chain), EXECUTOR (the CctpExecutor
///      address from DeployCctpExecutor — also identical on every chain), optional OWNER, optional
///      MAX_FEE_RATE (WAD). For a deterministic adapter address across chains, deploy from the same EOA
///      with identical args (same CONFIG, EXECUTOR, OWNER, MAX_FEE_RATE). Deploy the executor FIRST.
///      No post-deploy wiring is needed — domains/USDC/counterpart are all resolved from CONFIG at call time.
contract DeployCctpAdapter is Script {
    bytes32 internal constant SALT = keccak256("fast-fill.cctp.v1");

    function run() external returns (CctpAdapter adapter) {
        address config = vm.envAddress("CONFIG");
        address executor = vm.envAddress("EXECUTOR");
        address owner = vm.envOr("OWNER", msg.sender);
        uint256 maxFeeRate = vm.envOr("MAX_FEE_RATE", uint256(5e15)); // 0.5%

        vm.startBroadcast();
        adapter = new CctpAdapter{salt: SALT}(config, owner, maxFeeRate, executor);
        vm.stopBroadcast();

        console2.log("CctpAdapter:", address(adapter));
        console2.log("config:", config);
        console2.log("owner:", owner);
        console2.log("executor:", executor);
    }
}
