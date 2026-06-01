// SPDX-License-Identifier: MIT
pragma solidity ^0.8.26;

import {Script, console2} from "forge-std/Script.sol";
import {CctpAdapter} from "../src/adapters/CctpAdapter.sol";
import {Addresses} from "./config/Addresses.sol";

/// @notice Deploy a CctpAdapter wired to the canonical CCTP v2 contracts.
/// @dev Env: USDC (address), optional OWNER (address), optional MAX_FEE_RATE (uint, WAD).
///      Post-deploy, the owner must call setDomain(...) and setRemoteAdapter(...) for each
///      counterpart chain (see README).
contract DeployCctpAdapter is Script {
    function run() external returns (CctpAdapter adapter) {
        address owner = vm.envOr("OWNER", msg.sender);
        uint256 maxFeeRate = vm.envOr("MAX_FEE_RATE", uint256(5e15)); // 0.5%
        address usdc = vm.envAddress("USDC");

        vm.startBroadcast();
        adapter = new CctpAdapter(
            owner, maxFeeRate, Addresses.CCTP_TOKEN_MESSENGER_V2, Addresses.CCTP_MESSAGE_TRANSMITTER_V2, usdc
        );
        vm.stopBroadcast();

        console2.log("CctpAdapter:", address(adapter));
        console2.log("owner:", owner);
        console2.log("usdc:", usdc);
    }
}
