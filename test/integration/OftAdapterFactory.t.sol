// SPDX-License-Identifier: MIT
pragma solidity ^0.8.26;

import {Test} from "forge-std/Test.sol";
import {OftAdapterFactory} from "../../src/adapters/OftAdapterFactory.sol";
import {OftAdapter} from "../../src/adapters/OftAdapter.sol";
import {OftId} from "../../src/libraries/OftId.sol";
import {MockFastFillConfig} from "../mocks/MockFastFillConfig.sol";
import {MockOFT} from "../mocks/MockOFT.sol";
import {MockLzEndpoint} from "../mocks/MockLzEndpoint.sol";
import {ChainConfig, OftDeployment} from "../../src/interfaces/IFastFillConfig.sol";

/// @notice The factory is the "deploy a new OFT in one call" surface. These tests pin the properties
///         the fast-fill security model leans on: the adapter address is fully determined by the
///         (factory, oftId) pair (so it can be predicted and is stable across chains), each oftId is a
///         distinct, isolated deployment, and the shared config/owner/fee are baked in correctly.
contract OftAdapterFactoryTest is Test {
    MockFastFillConfig internal config;
    OftAdapterFactory internal factory;
    MockLzEndpoint internal endpoint;
    MockOFT internal oftToken;

    address internal owner = makeAddr("owner");
    uint256 internal constant MAX_FEE_RATE = 5e15;
    uint32 internal constant CHAIN = 1;
    uint32 internal constant EID = 30_101;

    function setUp() public {
        config = new MockFastFillConfig();
        // The adapter constructor now resolves + cross-checks its local OFT against the registry, so the
        // factory must deploy on a configured chain. Every tested oftId resolves to one shared mock OFT;
        // the adapters differ only by the oftId baked into their CREATE2 salt, so addresses stay distinct.
        endpoint = new MockLzEndpoint(EID);
        oftToken = new MockOFT(address(endpoint));
        config.set(CHAIN, ChainConfig(true, 0, EID, address(0), address(0)));
        config.setOft(CHAIN, OftId.USDT0, OftDeployment(address(oftToken), address(oftToken)));
        config.setOft(CHAIN, OftId.USDE, OftDeployment(address(oftToken), address(oftToken)));
        config.setOft(CHAIN, OftId.SUSDE, OftDeployment(address(oftToken), address(oftToken)));
        config.setOft(CHAIN, OftId.ENA, OftDeployment(address(oftToken), address(oftToken)));
        config.setOft(CHAIN, OftId.USDTB, OftDeployment(address(oftToken), address(oftToken)));
        vm.chainId(CHAIN);

        factory = new OftAdapterFactory(address(config), owner, MAX_FEE_RATE);
    }

    function test_deploy_matchesPredictionAndBakesImmutables() public {
        address predicted = factory.adapterFor(OftId.USDE);
        assertFalse(factory.isDeployed(OftId.USDE), "not deployed before");

        OftAdapter a = factory.deploy(OftId.USDE);

        assertEq(address(a), predicted, "deployed address == predicted");
        assertTrue(factory.isDeployed(OftId.USDE), "deployed after");
        assertEq(a.oftId(), OftId.USDE, "oftId baked in");
        assertEq(address(a.config()), address(config), "config baked in");
        assertEq(a.owner(), owner, "owner baked in");
        assertEq(a.maxFeeRate(), MAX_FEE_RATE, "maxFeeRate baked in");
    }

    function test_distinctOftIds_giveDistinctIsolatedAddresses() public {
        address usde = address(factory.deploy(OftId.USDE));
        address susde = address(factory.deploy(OftId.SUSDE));
        address ena = address(factory.deploy(OftId.ENA));

        assertTrue(usde != susde && susde != ena && usde != ena, "one address per oftId");
        // Each prediction is stable and matches what was deployed.
        assertEq(usde, factory.adapterFor(OftId.USDE), "usde prediction stable");
        assertEq(susde, factory.adapterFor(OftId.SUSDE), "susde prediction stable");
        assertEq(ena, factory.adapterFor(OftId.ENA), "ena prediction stable");
    }

    function test_redeploySameId_reverts() public {
        factory.deploy(OftId.ENA);
        // Second deploy targets the same CREATE2 address -> reverts.
        vm.expectRevert();
        factory.deploy(OftId.ENA);
    }

    function test_predictionIndependentOfDeployment() public view {
        // adapterFor is pure address math over (factory, salt, initcode) — valid before any deploy.
        assertTrue(factory.adapterFor(OftId.USDTB) != address(0), "predictable pre-deploy");
        assertTrue(
            factory.adapterFor(OftId.USDT0) != factory.adapterFor(OftId.USDTB), "different ids predict differently"
        );
    }
}
