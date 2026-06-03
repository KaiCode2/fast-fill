// SPDX-License-Identifier: MIT
pragma solidity ^0.8.26;

import {Script, console2} from "forge-std/Script.sol";
import {OftAdapterFactory} from "../src/adapters/OftAdapterFactory.sol";

/// @notice CREATE2-deploy the OftAdapterFactory against the deterministic FastFillConfig registry.
/// @dev Env: CONFIG (the FastFillConfig address — identical on every chain), optional OWNER (the
///      owner set on every adapter the factory deploys), optional MAX_FEE_RATE (WAD). Deploy from the
///      same EOA with identical args on every chain so the factory — and therefore every adapter it
///      creates — lands at the same address everywhere. Then run DeployOftAdapters once.
contract DeployOftAdapterFactory is Script {
    bytes32 internal constant SALT = keccak256("fast-fill.oft-factory.v1");

    function run() external returns (OftAdapterFactory factory) {
        address config = vm.envAddress("CONFIG");
        address owner = vm.envOr("OWNER", msg.sender);
        uint256 maxFeeRate = vm.envOr("MAX_FEE_RATE", uint256(5e15)); // 0.5%

        vm.startBroadcast();
        factory = new OftAdapterFactory{salt: SALT}(config, owner, maxFeeRate);
        vm.stopBroadcast();

        console2.log("OftAdapterFactory:", address(factory));
        console2.log("config:", config);
        console2.log("owner:", owner);
    }
}
