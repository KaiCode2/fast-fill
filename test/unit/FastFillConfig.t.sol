// SPDX-License-Identifier: MIT
pragma solidity ^0.8.26;

import {Test} from "forge-std/Test.sol";
import {FastFillConfig} from "../../src/config/FastFillConfig.sol";
import {ChainConfig, OftDeployment} from "../../src/interfaces/IFastFillConfig.sol";
import {OftId} from "../../src/libraries/OftId.sol";
import {Addresses} from "../../script/config/Addresses.sol";

/// @notice Binds the production registry to the canonical address book so the two can never drift,
///         and documents the per-chain rows (transport + every OFT deployment). The fork tests
///         additionally check the registry against the live chains.
contract FastFillConfigTest is Test {
    FastFillConfig internal config;

    function setUp() public {
        config = new FastFillConfig();
    }

    function _assertOft(uint256 chainId, uint8 oftId, address oft, address token) internal view {
        OftDeployment memory d = config.oftConfig(chainId, oftId);
        assertEq(d.oft, oft, "oft");
        assertEq(d.token, token, "token");
    }

    function test_ethereum() public view {
        ChainConfig memory c = config.chainConfig(1);
        assertTrue(c.supported, "supported");
        assertEq(c.cctpDomain, Addresses.DOMAIN_ETHEREUM, "domain");
        assertEq(c.lzEid, Addresses.EID_ETHEREUM, "eid");
        assertEq(c.usdc, Addresses.USDC_ETHEREUM, "usdc");
        assertEq(c.cctpTokenMessenger, Addresses.CCTP_TOKEN_MESSENGER_V2, "messenger");
        // OFTs: Ethereum is the adapter/lockbox chain, so oft != token for all of them.
        _assertOft(1, OftId.USDT0, Addresses.USDT0_OFT_ETHEREUM, Addresses.USDT0_TOKEN_ETHEREUM);
        _assertOft(1, OftId.USDE, Addresses.USDE_OFT_ETHEREUM, Addresses.USDE_TOKEN_ETHEREUM);
        _assertOft(1, OftId.SUSDE, Addresses.SUSDE_OFT_ETHEREUM, Addresses.SUSDE_TOKEN_ETHEREUM);
        _assertOft(1, OftId.ENA, Addresses.ENA_OFT_ETHEREUM, Addresses.ENA_TOKEN_ETHEREUM);
        _assertOft(1, OftId.USDTB, Addresses.USDTB_OFT_ETHEREUM, Addresses.USDTB_TOKEN_ETHEREUM);
    }

    function test_optimism() public view {
        ChainConfig memory c = config.chainConfig(10);
        assertTrue(c.supported, "supported");
        assertEq(c.cctpDomain, Addresses.DOMAIN_OPTIMISM, "domain");
        assertEq(c.lzEid, Addresses.EID_OPTIMISM, "eid");
        assertEq(c.usdc, Addresses.USDC_OPTIMISM, "usdc");
        // L2: native OFTs (oft == token). USDtb is NOT on Optimism.
        _assertOft(10, OftId.USDT0, Addresses.USDT0_OFT_OPTIMISM, Addresses.USDT0_TOKEN_OPTIMISM);
        _assertOft(10, OftId.USDE, Addresses.USDE_OPTIMISM, Addresses.USDE_OPTIMISM);
        _assertOft(10, OftId.SUSDE, Addresses.SUSDE_OPTIMISM, Addresses.SUSDE_OPTIMISM);
        _assertOft(10, OftId.ENA, Addresses.ENA_OPTIMISM, Addresses.ENA_OPTIMISM);
        _assertOft(10, OftId.USDTB, address(0), address(0)); // not deployed on Optimism
    }

    function test_arbitrum() public view {
        ChainConfig memory c = config.chainConfig(42_161);
        assertTrue(c.supported, "supported");
        assertEq(c.cctpDomain, Addresses.DOMAIN_ARBITRUM, "domain");
        assertEq(c.lzEid, Addresses.EID_ARBITRUM, "eid");
        assertEq(c.usdc, Addresses.USDC_ARBITRUM, "usdc");
        _assertOft(42_161, OftId.USDT0, Addresses.USDT0_OFT_ARBITRUM, Addresses.USDT0_TOKEN_ARBITRUM);
        _assertOft(42_161, OftId.USDE, Addresses.USDE_ARBITRUM, Addresses.USDE_ARBITRUM);
        _assertOft(42_161, OftId.SUSDE, Addresses.SUSDE_ARBITRUM, Addresses.SUSDE_ARBITRUM);
        _assertOft(42_161, OftId.ENA, Addresses.ENA_ARBITRUM, Addresses.ENA_ARBITRUM);
        _assertOft(42_161, OftId.USDTB, Addresses.USDTB_ARBITRUM, Addresses.USDTB_ARBITRUM);
    }

    function test_base() public view {
        ChainConfig memory c = config.chainConfig(8453);
        assertTrue(c.supported, "supported");
        assertEq(c.cctpDomain, Addresses.DOMAIN_BASE, "domain");
        assertEq(c.lzEid, Addresses.EID_BASE, "eid");
        assertEq(c.usdc, Addresses.USDC_BASE, "usdc");
        // USD₮0 is NOT on Base; the Ethena suite is (all native).
        _assertOft(8453, OftId.USDT0, address(0), address(0)); // no USD₮0 on Base
        _assertOft(8453, OftId.USDE, Addresses.USDE_BASE, Addresses.USDE_BASE);
        _assertOft(8453, OftId.SUSDE, Addresses.SUSDE_BASE, Addresses.SUSDE_BASE);
        _assertOft(8453, OftId.ENA, Addresses.ENA_BASE, Addresses.ENA_BASE);
        _assertOft(8453, OftId.USDTB, Addresses.USDTB_BASE, Addresses.USDTB_BASE);
    }

    function test_unknownChain_unsupported() public view {
        assertFalse(config.chainConfig(123_456).supported, "unknown chain unsupported");
        _assertOft(123_456, OftId.USDE, address(0), address(0)); // unknown chain has no OFTs
    }

    function test_unknownOftId_unset() public view {
        _assertOft(1, 200, address(0), address(0)); // unassigned id on a known chain
    }
}
