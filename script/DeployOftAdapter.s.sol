// SPDX-License-Identifier: MIT
pragma solidity ^0.8.26;

import {Script, console2} from "forge-std/Script.sol";
import {OftAdapter} from "../src/adapters/OftAdapter.sol";

/// @notice CREATE2-deploy an OftAdapter (USD₮0) against the deterministic FastFillConfig registry.
/// @dev Env: CONFIG (the FastFillConfig address — identical on every chain), optional OWNER,
///      optional MAX_FEE_RATE (WAD). For a deterministic adapter address across chains, deploy from
///      the same EOA with identical args. No post-deploy wiring is needed — the OFT/token/eid are
///      resolved from CONFIG (and read live from the OFT) at call time.
contract DeployOftAdapter is Script {
    bytes32 internal constant SALT = keccak256("fast-fill.oft.v1");

    function run() external returns (OftAdapter adapter) {
        address config = vm.envAddress("CONFIG");
        address owner = vm.envOr("OWNER", msg.sender);
        uint256 maxFeeRate = vm.envOr("MAX_FEE_RATE", uint256(5e15)); // 0.5%

        vm.startBroadcast();
        adapter = new OftAdapter{salt: SALT}(config, owner, maxFeeRate);
        vm.stopBroadcast();

        console2.log("OftAdapter:", address(adapter));
        console2.log("config:", config);
        console2.log("owner:", owner);
    }
}
