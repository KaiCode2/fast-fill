// SPDX-License-Identifier: MIT
pragma solidity ^0.8.26;

import {ForkBase} from "./ForkBase.sol";
import {Addresses} from "../../script/config/Addresses.sol";

/// @notice Fork check against the real LayerZero v2 endpoint. Self-skips when ETH_RPC_URL is unset.
///
///         Note: an end-to-end OFT compose cannot complete on a single fork (it needs the source
///         and destination chains plus the DVN/executor). Use the local lifecycle tests for the
///         compose flow; this only asserts the endpoint is where we expect on a real network.
contract OftForkTest is ForkBase {
    function test_fork_endpointDeployed() external {
        if (!_forkMainnetOrSkip()) return;
        assertGt(Addresses.LZ_ENDPOINT_V2.code.length, 0, "EndpointV2 has code");
    }
}
