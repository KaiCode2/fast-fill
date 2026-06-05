// SPDX-License-Identifier: MIT
pragma solidity ^0.8.26;

import {Fixtures} from "../utils/Fixtures.sol";
import {FastFillBase} from "../../src/FastFillBase.sol";
import {CctpAdapter} from "../../src/adapters/CctpAdapter.sol";
import {CctpExecutor} from "../../src/CctpExecutor.sol";
import {OftAdapter} from "../../src/adapters/OftAdapter.sol";
import {MockFastFillConfig} from "../mocks/MockFastFillConfig.sol";
import {MockMessageTransmitterV2} from "../mocks/MockMessageTransmitterV2.sol";
import {MockTokenMessengerV2} from "../mocks/MockTokenMessengerV2.sol";
import {MockOFT} from "../mocks/MockOFT.sol";
import {MockLzEndpoint} from "../mocks/MockLzEndpoint.sol";
import {ChainConfig, OftDeployment} from "../../src/interfaces/IFastFillConfig.sol";
import {OftId} from "../../src/libraries/OftId.sol";

/// @notice The adapters and the executor resolve their LOCAL chain config ONCE at construction and cache
///         it in immutables, so the hot paths no longer pay a cold registry view call. These tests pin
///         the two properties that change buys: the cached locals match the registry, and a wrong
///         registry constant fails CLOSED at deploy — the live cross-check moved from every call to the
///         constructor (so a misconfiguration can never silently ship).
contract ConfigCachingTest is Fixtures {
    function setUp() public {
        vm.warp(1_000_000);
        _setUpCctp();
        _setUpOft();
    }

    function test_cctp_cachesLocalConfig() public view {
        assertEq(cctp.usdc(), address(usdc), "cctp usdc cached");
        assertEq(cctp.cctpTokenMessenger(), address(tokenMessenger), "cctp messenger cached");
        assertEq(cctp.cctpTransmitter(), address(transmitter), "cctp transmitter cached");
        assertEq(cctpExec.usdc(), address(usdc), "executor usdc cached");
        assertEq(cctpExec.transmitter(), address(transmitter), "executor transmitter cached");
    }

    function test_oft_cachesLocalConfig() public view {
        assertEq(oft.localOft(), address(oftToken), "oft entrypoint cached");
        assertEq(oft.localToken(), address(oftToken), "oft token cached");
        assertEq(oft.localEndpoint(), address(lzEndpoint), "oft endpoint cached");
    }

    function test_cctp_domainMismatch_revertsAtConstruction() public {
        // Registry claims SRC_DOMAIN, but the live transmitter reports a different domain.
        MockMessageTransmitterV2 t = new MockMessageTransmitterV2(address(usdc), 99);
        MockTokenMessengerV2 m = new MockTokenMessengerV2(address(t));
        MockFastFillConfig cfg = new MockFastFillConfig();
        cfg.set(SRC_CHAIN, ChainConfig(true, SRC_DOMAIN, 0, address(usdc), address(m)));
        vm.chainId(SRC_CHAIN);
        vm.expectRevert(abi.encodeWithSelector(CctpAdapter.DomainMismatch.selector, SRC_DOMAIN, uint32(99)));
        new CctpAdapter(address(cfg), owner, MAX_FEE_RATE, address(cctpExec));
    }

    function test_cctp_unsupportedChain_revertsAtConstruction() public {
        MockFastFillConfig cfg = new MockFastFillConfig(); // nothing registered for this chain
        vm.chainId(SRC_CHAIN);
        vm.expectRevert(abi.encodeWithSelector(FastFillBase.UnsupportedChain.selector, uint32(SRC_CHAIN)));
        new CctpAdapter(address(cfg), owner, MAX_FEE_RATE, address(cctpExec));
    }

    function test_oft_eidMismatch_revertsAtConstruction() public {
        // Live endpoint reports eid 99, registry says SRC_EID.
        MockLzEndpoint ep = new MockLzEndpoint(99);
        MockOFT o = new MockOFT(address(ep));
        MockFastFillConfig cfg = new MockFastFillConfig();
        cfg.set(SRC_CHAIN, ChainConfig(true, 0, SRC_EID, address(0), address(0)));
        cfg.setOft(SRC_CHAIN, OftId.USDT0, OftDeployment(address(o), address(o)));
        vm.chainId(SRC_CHAIN);
        vm.expectRevert(abi.encodeWithSelector(OftAdapter.EidMismatch.selector, SRC_EID, uint32(99)));
        new OftAdapter(address(cfg), owner, MAX_FEE_RATE, OftId.USDT0);
    }

    function test_oft_tokenMismatch_revertsAtConstruction() public {
        // Registry's token != the live OFT's token().
        address wrongToken = makeAddr("wrongToken");
        cfgWithOftToken(wrongToken);
    }

    function cfgWithOftToken(address registryToken) internal {
        MockLzEndpoint ep = new MockLzEndpoint(SRC_EID);
        MockOFT o = new MockOFT(address(ep)); // o.token() == address(o)
        MockFastFillConfig cfg = new MockFastFillConfig();
        cfg.set(SRC_CHAIN, ChainConfig(true, 0, SRC_EID, address(0), address(0)));
        cfg.setOft(SRC_CHAIN, OftId.USDT0, OftDeployment(address(o), registryToken));
        vm.chainId(SRC_CHAIN);
        vm.expectRevert(abi.encodeWithSelector(OftAdapter.TokenMismatch.selector, address(o), registryToken));
        new OftAdapter(address(cfg), owner, MAX_FEE_RATE, OftId.USDT0);
    }
}
