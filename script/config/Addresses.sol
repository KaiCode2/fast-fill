// SPDX-License-Identifier: MIT
pragma solidity ^0.8.26;

/// @notice Canonical mainnet addresses and transport ids for CCTP v2 and LayerZero v2.
/// @dev CCTP v2 contracts are CREATE2-deployed to the same address on every supported EVM chain.
///      USDC is per-chain. Verify against developers.circle.com/cctp/evm-smart-contracts and the
///      LayerZero deployments page before any production deploy.
library Addresses {
    // --- CCTP v2 (same address across chains) ---
    address internal constant CCTP_TOKEN_MESSENGER_V2 = 0x28b5a0e9C621a5BadaA536219b3a228C8168cf5d;
    address internal constant CCTP_MESSAGE_TRANSMITTER_V2 = 0x81D40F21F12A8F0E3252Bccb954D722d4c464B64;

    // --- USDC (per chain) ---
    address internal constant USDC_ETHEREUM = 0xA0b86991c6218b36c1d19D4a2e9Eb0cE3606eB48;
    address internal constant USDC_BASE = 0x833589fCD6eDb6E08f4c7C32D4f71b54bdA02913;
    address internal constant USDC_ARBITRUM = 0xaf88d065e77c8cC2239327C5EDb3A432268e5831;

    // --- LayerZero v2 EndpointV2 (same address across chains) ---
    address internal constant LZ_ENDPOINT_V2 = 0x1a44076050125825900e736c501f859c50fE728c;

    // --- EVM chain ids ---
    uint32 internal constant CHAIN_ETHEREUM = 1;
    uint32 internal constant CHAIN_OPTIMISM = 10;
    uint32 internal constant CHAIN_ARBITRUM = 42_161;
    uint32 internal constant CHAIN_BASE = 8453;
    uint32 internal constant CHAIN_AVALANCHE = 43_114;

    // --- CCTP domains ---
    uint32 internal constant DOMAIN_ETHEREUM = 0;
    uint32 internal constant DOMAIN_AVALANCHE = 1;
    uint32 internal constant DOMAIN_OPTIMISM = 2;
    uint32 internal constant DOMAIN_ARBITRUM = 3;
    uint32 internal constant DOMAIN_BASE = 6;

    // --- LayerZero endpoint ids ---
    uint32 internal constant EID_ETHEREUM = 30_101;
    uint32 internal constant EID_ARBITRUM = 30_110;
    uint32 internal constant EID_BASE = 30_184;
}
