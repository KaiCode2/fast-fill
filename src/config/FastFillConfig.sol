// SPDX-License-Identifier: MIT
pragma solidity ^0.8.26;

import {IFastFillConfig, ChainConfig} from "../interfaces/IFastFillConfig.sol";

/// @title  FastFillConfig
/// @notice The canonical, immutable chain registry for fast-fill. Every supported chain's bridge
///         addresses and transport ids are baked in as constants, so the bytecode is identical on
///         every chain and the contract can be CREATE2-deployed to one deterministic address
///         everywhere. Adapters take this single address and resolve all chain-specific data from
///         it — there are no owner setters and no per-deployment configuration to get wrong.
///
///         What is NOT baked is anything an adapter can read live on its own chain: the CCTP
///         MessageTransmitter (read from the TokenMessenger), the local CCTP domain (read from the
///         MessageTransmitter), and the OFT endpoint / token / eid (read from the OFT). Adapters
///         cross-check those live reads against this registry, so a wrong constant here cannot
///         silently ship — deployment/operation reverts on mismatch.
///
///         Sources: Circle CCTP v2 (developers.circle.com/cctp), LayerZero v2 deployments, and
///         USD₮0 (github.com/Everdawn-Labs/usdt0-audit-reports/blob/main/DEPLOYMENTS.md). All
///         values are additionally verified on-chain in the fork tests.
contract FastFillConfig is IFastFillConfig {
    // --- CCTP v2 TokenMessenger (CREATE2-identical across every supported chain) ---
    address internal constant CCTP_TOKEN_MESSENGER_V2 = 0x28b5a0e9C621a5BadaA536219b3a228C8168cf5d;

    // --- USDC (per chain) ---
    address internal constant USDC_ETHEREUM = 0xA0b86991c6218b36c1d19D4a2e9Eb0cE3606eB48;
    address internal constant USDC_OPTIMISM = 0x0b2C639c533813f4Aa9D7837CAf62653d097Ff85;
    address internal constant USDC_ARBITRUM = 0xaf88d065e77c8cC2239327C5EDb3A432268e5831;
    address internal constant USDC_BASE = 0x833589fCD6eDb6E08f4c7C32D4f71b54bdA02913;

    // --- USD₮0 OFT entrypoint (per chain; Ethereum is the lockbox OFTAdapter over USDT) ---
    address internal constant USDT0_OFT_ETHEREUM = 0x6C96dE32CEa08842dcc4058c14d3aaAD7Fa41dee;
    address internal constant USDT0_OFT_OPTIMISM = 0xF03b4d9AC1D5d1E7c4cEf54C2A313b9fe051A0aD;
    address internal constant USDT0_OFT_ARBITRUM = 0x14E4A1B13bf7F943c8ff7C51fb60FA964A298D92;

    // --- USD₮0 token delivered on each chain (Ethereum delivers real USDT) ---
    address internal constant USDT0_TOKEN_ETHEREUM = 0xdAC17F958D2ee523a2206206994597C13D831ec7;
    address internal constant USDT0_TOKEN_OPTIMISM = 0x01bFF41798a0BcF287b996046Ca68b395DbC1071;
    address internal constant USDT0_TOKEN_ARBITRUM = 0xFd086bC7CD5C481DCC9C85ebE478A1C0b69FCbb9;

    /// @inheritdoc IFastFillConfig
    function chainConfig(uint256 chainId) external pure returns (ChainConfig memory) {
        // Ethereum
        if (chainId == 1) {
            return ChainConfig(
                true, 0, 30_101, USDC_ETHEREUM, CCTP_TOKEN_MESSENGER_V2, USDT0_OFT_ETHEREUM, USDT0_TOKEN_ETHEREUM
            );
        }
        // Optimism
        if (chainId == 10) {
            return ChainConfig(
                true, 2, 30_111, USDC_OPTIMISM, CCTP_TOKEN_MESSENGER_V2, USDT0_OFT_OPTIMISM, USDT0_TOKEN_OPTIMISM
            );
        }
        // Arbitrum One
        if (chainId == 42_161) {
            return ChainConfig(
                true, 3, 30_110, USDC_ARBITRUM, CCTP_TOKEN_MESSENGER_V2, USDT0_OFT_ARBITRUM, USDT0_TOKEN_ARBITRUM
            );
        }
        // Base — CCTP/USDC only; USD₮0 is not deployed here.
        if (chainId == 8453) {
            return ChainConfig(true, 6, 30_184, USDC_BASE, CCTP_TOKEN_MESSENGER_V2, address(0), address(0));
        }
        // Unknown chain.
        return ChainConfig(false, 0, 0, address(0), address(0), address(0), address(0));
    }
}
