// SPDX-License-Identifier: MIT
pragma solidity ^0.8.26;

import {Script, console2} from "forge-std/Script.sol";
import {OftAdapterFactory} from "../src/adapters/OftAdapterFactory.sol";
import {OftId} from "../src/libraries/OftId.sol";

/// @notice Deploy the supported OftAdapters via the already-deployed OftAdapterFactory.
/// @dev Env: FACTORY (the OftAdapterFactory address — identical on every chain). The OFTs deployed
///      here are fixed in-script: USD₮0, sUSDe, USDe, and ENA. The factory bakes
///      config/owner/maxFeeRate, and the resulting adapters land at the same addresses on every
///      chain. No post-deploy wiring — the OFT/token/eid are resolved from the registry (and read
///      live from the OFT) at call time.
contract DeployOftAdapters is Script {
    function run() external returns (address[4] memory adapters) {
        OftAdapterFactory factory = OftAdapterFactory(vm.envAddress("FACTORY"));
        uint8[4] memory oftIds = _oftIds();
        string[4] memory names = _names();
        address[4] memory predicted;
        bool[4] memory deployed;

        for (uint256 i; i < oftIds.length; ++i) {
            predicted[i] = factory.adapterFor(oftIds[i]);
        }

        vm.startBroadcast();
        for (uint256 i; i < oftIds.length; ++i) {
            if (predicted[i].code.length == 0) {
                adapters[i] = address(factory.deploy(oftIds[i]));
                deployed[i] = true;
            } else {
                adapters[i] = predicted[i];
            }
        }
        vm.stopBroadcast();

        console2.log("factory:", address(factory));
        for (uint256 i; i < oftIds.length; ++i) {
            require(adapters[i] == predicted[i], "adapter address mismatch");
            console2.log(names[i], "OftAdapter:", adapters[i]);
            console2.log("oftId:", oftIds[i]);
            console2.log("status:", deployed[i] ? "deployed" : "already deployed");
        }
    }

    function _oftIds() internal pure returns (uint8[4] memory ids) {
        ids = [OftId.USDT0, OftId.SUSDE, OftId.USDE, OftId.ENA];
    }

    function _names() internal pure returns (string[4] memory names) {
        names = ["USDT0", "sUSDe", "USDe", "ENA"];
    }
}

/// @dev Backwards-compatible script contract name for existing forge invocations.
contract DeployOftAdapter is DeployOftAdapters {}
