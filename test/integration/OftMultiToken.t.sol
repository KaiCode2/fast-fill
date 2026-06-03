// SPDX-License-Identifier: MIT
pragma solidity ^0.8.26;

import {OftLifecycleTest} from "./OftLifecycle.t.sol";
import {OftId} from "../../src/libraries/OftId.sol";

/// @notice Re-runs the ENTIRE USD₮0 OFT lifecycle suite against an adapter deployed for a DIFFERENT
///         OFT (USDe). The adapter resolves its (oft, token) purely from `oftConfig(chainid, oftId)`,
///         and the `oftId` is NOT part of the order, so the full source/fill/settle/adversarial
///         lifecycle is OFT-agnostic. This file proves that by inheriting every OftLifecycleTest case
///         and only swapping the setup id — if any path were secretly USD₮0-specific, it would fail here.
contract OftMultiTokenTest is OftLifecycleTest {
    function setUp() public override {
        vm.warp(1_000_000);
        _setUpOftFor(OftId.USDE);
    }

    function test_adapter_boundToUsdeId() public view {
        assertEq(oft.oftId(), OftId.USDE, "adapter bound to the USDe oftId, not USDT0");
    }
}
