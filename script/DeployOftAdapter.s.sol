// SPDX-License-Identifier: MIT
pragma solidity ^0.8.26;

import {Script, console2} from "forge-std/Script.sol";
import {OftAdapter} from "../src/adapters/OftAdapter.sol";
import {Addresses} from "./config/Addresses.sol";

/// @notice Deploy an OftAdapter wired to the canonical LayerZero v2 endpoint and a given OFT.
/// @dev Env: OFT (address of the OFT this adapter bridges), optional OWNER, optional MAX_FEE_RATE.
///      Post-deploy, the owner must call setEid(...) and setRemoteAdapter(...) for each counterpart
///      chain (see README).
contract DeployOftAdapter is Script {
    function run() external returns (OftAdapter adapter) {
        address owner = vm.envOr("OWNER", msg.sender);
        uint256 maxFeeRate = vm.envOr("MAX_FEE_RATE", uint256(5e15)); // 0.5%
        address oft = vm.envAddress("OFT");

        vm.startBroadcast();
        adapter = new OftAdapter(owner, maxFeeRate, Addresses.LZ_ENDPOINT_V2, oft);
        vm.stopBroadcast();

        console2.log("OftAdapter:", address(adapter));
        console2.log("owner:", owner);
        console2.log("oft:", oft);
    }
}
