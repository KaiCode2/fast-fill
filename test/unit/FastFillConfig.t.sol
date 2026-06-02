// SPDX-License-Identifier: MIT
pragma solidity ^0.8.26;

import {Test} from "forge-std/Test.sol";
import {FastFillConfig} from "../../src/config/FastFillConfig.sol";
import {ChainConfig} from "../../src/interfaces/IFastFillConfig.sol";
import {Addresses} from "../../script/config/Addresses.sol";

/// @notice Binds the production registry to the canonical address book so the two can never drift,
///         and documents the per-chain rows. (The fork tests additionally check the registry
///         against the live chains.)
contract FastFillConfigTest is Test {
    FastFillConfig internal config;

    function setUp() public {
        config = new FastFillConfig();
    }

    function test_ethereum() public view {
        ChainConfig memory c = config.chainConfig(1);
        assertTrue(c.supported, "supported");
        assertEq(c.cctpDomain, Addresses.DOMAIN_ETHEREUM, "domain");
        assertEq(c.lzEid, Addresses.EID_ETHEREUM, "eid");
        assertEq(c.usdc, Addresses.USDC_ETHEREUM, "usdc");
        assertEq(c.cctpTokenMessenger, Addresses.CCTP_TOKEN_MESSENGER_V2, "messenger");
        assertEq(c.usdt0Oft, Addresses.USDT0_OFT_ETHEREUM, "oft");
        assertEq(c.usdt0Token, Addresses.USDT0_TOKEN_ETHEREUM, "token");
    }

    function test_optimism() public view {
        ChainConfig memory c = config.chainConfig(10);
        assertTrue(c.supported, "supported");
        assertEq(c.cctpDomain, Addresses.DOMAIN_OPTIMISM, "domain");
        assertEq(c.lzEid, Addresses.EID_OPTIMISM, "eid");
        assertEq(c.usdc, Addresses.USDC_OPTIMISM, "usdc");
        assertEq(c.usdt0Oft, Addresses.USDT0_OFT_OPTIMISM, "oft");
        assertEq(c.usdt0Token, Addresses.USDT0_TOKEN_OPTIMISM, "token");
    }

    function test_arbitrum() public view {
        ChainConfig memory c = config.chainConfig(42_161);
        assertTrue(c.supported, "supported");
        assertEq(c.cctpDomain, Addresses.DOMAIN_ARBITRUM, "domain");
        assertEq(c.lzEid, Addresses.EID_ARBITRUM, "eid");
        assertEq(c.usdc, Addresses.USDC_ARBITRUM, "usdc");
        assertEq(c.usdt0Oft, Addresses.USDT0_OFT_ARBITRUM, "oft");
        assertEq(c.usdt0Token, Addresses.USDT0_TOKEN_ARBITRUM, "token");
    }

    function test_base_cctpOnly_noUsdt0() public view {
        ChainConfig memory c = config.chainConfig(8453);
        assertTrue(c.supported, "supported");
        assertEq(c.cctpDomain, Addresses.DOMAIN_BASE, "domain");
        assertEq(c.usdc, Addresses.USDC_BASE, "usdc");
        assertEq(c.usdt0Oft, address(0), "no USD0 OFT on base");
        assertEq(c.usdt0Token, address(0), "no USD0 token on base");
    }

    function test_unknownChain_unsupported() public view {
        assertFalse(config.chainConfig(123_456).supported, "unknown chain unsupported");
    }
}
