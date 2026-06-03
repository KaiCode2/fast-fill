// SPDX-License-Identifier: MIT
pragma solidity ^0.8.26;

import {ForkBase} from "./ForkBase.sol";
import {Addresses} from "../../script/config/Addresses.sol";
import {FastFillConfig} from "../../src/config/FastFillConfig.sol";
import {OftId} from "../../src/libraries/OftId.sol";
import {OftDeployment} from "../../src/interfaces/IFastFillConfig.sol";
import {IOFT} from "../../src/interfaces/layerzero/IOFT.sol";

interface IERC20Decimals {
    function decimals() external view returns (uint8);
}

/// @notice Verifies the registry's Ethena OFT rows against the LIVE chains — the address-correctness
///         gate for the new OFTs (USDe, sUSDe, ENA, USDtb). For each (chain, token) we assert the
///         load-bearing facts our adapter relies on, read directly from the on-chain OFT:
///           - `OFT.token()` equals the ERC20 the registry says it delivers (so the adapter approves
///             and credits the right asset),
///           - `OFT.endpoint()` is the canonical LayerZero v2 EndpointV2,
///           - native vs adapter topology matches the registry (L2 native: oft == token; Ethereum
///             lockbox: oft != token),
///           - the ERC20 has 18 decimals (the Ethena suite).
///         A wrong/typo'd constant fails here loudly rather than in production. Each test self-skips
///         when its chain's RPC is not configured.
contract EthenaOftForkTest is ForkBase {
    FastFillConfig internal config;

    function _check(uint256 chainId, uint8 oftId, bool isAdapterChain) internal view {
        OftDeployment memory d = config.oftConfig(chainId, oftId);
        assertTrue(d.oft != address(0) && d.token != address(0), "registry has a deployment");

        // The adapter cross-checks exactly these two live reads, so assert them here too.
        assertEq(IOFT(d.oft).token(), d.token, "OFT.token() == registry token");
        assertEq(IOFT(d.oft).endpoint(), Addresses.LZ_ENDPOINT_V2, "OFT.endpoint() == canonical EndpointV2");

        if (isAdapterChain) {
            assertTrue(d.oft != d.token, "adapter/lockbox: entrypoint != ERC20");
        } else {
            assertEq(d.oft, d.token, "native OFT: entrypoint == ERC20");
        }

        // Ethena suite is 18-decimal everywhere. Read decimals from the ERC20 (the adapter's token()
        // entrypoint can revert on decimals()), not from the OFT entrypoint.
        assertEq(IERC20Decimals(d.token).decimals(), 18, "18 decimals");
    }

    function test_fork_eth_ethena_facts() external {
        if (!_forkOrSkip(_ethRpcUrl())) return;
        config = new FastFillConfig();
        _check(1, OftId.USDE, true);
        _check(1, OftId.SUSDE, true);
        _check(1, OftId.ENA, true);
        _check(1, OftId.USDTB, true);
    }

    function test_fork_op_ethena_facts() external {
        if (!_forkOrSkip(_opRpcUrl())) return;
        config = new FastFillConfig();
        _check(10, OftId.USDE, false);
        _check(10, OftId.SUSDE, false);
        _check(10, OftId.ENA, false);
        // USDtb is not deployed on Optimism (registry returns address(0)).
        OftDeployment memory usdtb = config.oftConfig(10, OftId.USDTB);
        assertEq(usdtb.oft, address(0), "USDtb absent on Optimism");
        assertEq(usdtb.token, address(0), "USDtb absent on Optimism");
    }

    function test_fork_arb_ethena_facts() external {
        if (!_forkOrSkip(_arbRpcUrl())) return;
        config = new FastFillConfig();
        _check(42_161, OftId.USDE, false);
        _check(42_161, OftId.SUSDE, false);
        _check(42_161, OftId.ENA, false);
        _check(42_161, OftId.USDTB, false);
    }

    function test_fork_base_ethena_facts() external {
        if (!_forkOrSkip(_baseRpcUrl())) return;
        config = new FastFillConfig();
        _check(8453, OftId.USDE, false);
        _check(8453, OftId.SUSDE, false);
        _check(8453, OftId.ENA, false);
        _check(8453, OftId.USDTB, false);
    }
}
