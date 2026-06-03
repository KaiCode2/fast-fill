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
    address internal constant USDC_OPTIMISM = 0x0b2C639c533813f4Aa9D7837CAf62653d097Ff85;
    address internal constant USDC_BASE = 0x833589fCD6eDb6E08f4c7C32D4f71b54bdA02913;
    address internal constant USDC_ARBITRUM = 0xaf88d065e77c8cC2239327C5EDb3A432268e5831;

    // --- LayerZero v2 EndpointV2 (same address across chains) ---
    address internal constant LZ_ENDPOINT_V2 = 0x1a44076050125825900e736c501f859c50fE728c;

    // --- USDT0 (LayerZero OFT). `OFT` is the proxy we call send() on; `TOKEN` is the ERC20 users
    //     hold (a DIFFERENT address per chain). Ethereum is the lockbox OFTAdapter over USDT.
    //     Source: github.com/Everdawn-Labs/usdt0-audit-reports/blob/main/DEPLOYMENTS.md ---
    address internal constant USDT0_OFT_ETHEREUM = 0x6C96dE32CEa08842dcc4058c14d3aaAD7Fa41dee;
    address internal constant USDT0_TOKEN_ETHEREUM = 0xdAC17F958D2ee523a2206206994597C13D831ec7; // USDT
    address internal constant USDT0_OFT_OPTIMISM = 0xF03b4d9AC1D5d1E7c4cEf54C2A313b9fe051A0aD;
    address internal constant USDT0_TOKEN_OPTIMISM = 0x01bFF41798a0BcF287b996046Ca68b395DbC1071;
    address internal constant USDT0_OFT_ARBITRUM = 0x14E4A1B13bf7F943c8ff7C51fb60FA964A298D92;
    address internal constant USDT0_TOKEN_ARBITRUM = 0xFd086bC7CD5C481DCC9C85ebE478A1C0b69FCbb9;

    // --- Ethena OFTs (LayerZero v2). All four use the same topology as USD₮0: an adapter/lockbox on
    //     Ethereum where the OFT entrypoint (`OFT`) and the ERC20 (`TOKEN`) are DIFFERENT addresses,
    //     and native mint/burn OFTs on the L2s where the entrypoint IS the ERC20 (so `OFT == TOKEN`).
    //     Sources: docs.ethena.fi/solution-design/key-addresses, docs.usdtb.money/key-addresses, and
    //     the LayerZero metadata API. Re-verify on a fork (EthenaOftFork.t.sol) before any prod deploy. ---

    // USDe — 18 decimals. Deployed on all four chains.
    address internal constant USDE_OFT_ETHEREUM = 0x5d3a1Ff2b6BAb83b63cd9AD0787074081a52ef34;
    address internal constant USDE_TOKEN_ETHEREUM = 0x4c9EDD5852cd905f086C759E8383e09bff1E68B3;
    address internal constant USDE_OPTIMISM = 0x5d3a1Ff2b6BAb83b63cd9AD0787074081a52ef34; // native: OFT == TOKEN
    address internal constant USDE_ARBITRUM = 0x5d3a1Ff2b6BAb83b63cd9AD0787074081a52ef34;
    address internal constant USDE_BASE = 0x5d3a1Ff2b6BAb83b63cd9AD0787074081a52ef34;

    // sUSDe (Staked USDe) — 18 decimals. All four chains. NB: the ERC20 enforces a transfer blacklist,
    // so a blocked recipient/filler is routed to the pull-payment claim ledger (settlement still completes).
    address internal constant SUSDE_OFT_ETHEREUM = 0x211Cc4DD073734dA055fbF44a2b4667d5E5fE5d2;
    address internal constant SUSDE_TOKEN_ETHEREUM = 0x9D39A5DE30e57443BfF2A8307A4256c8797A3497;
    address internal constant SUSDE_OPTIMISM = 0x211Cc4DD073734dA055fbF44a2b4667d5E5fE5d2;
    address internal constant SUSDE_ARBITRUM = 0x211Cc4DD073734dA055fbF44a2b4667d5E5fE5d2;
    address internal constant SUSDE_BASE = 0x211Cc4DD073734dA055fbF44a2b4667d5E5fE5d2;

    // ENA (governance) — 18 decimals. All four chains. NB: the L2 OFT address differs from the
    // Ethereum adapter address (unlike USDe/sUSDe, whose adapter & L2 OFT share one address).
    address internal constant ENA_OFT_ETHEREUM = 0x90967a0Af8178ec6024Ab15461Ed79B8676071D0;
    address internal constant ENA_TOKEN_ETHEREUM = 0x57e114B691Db790C35207b2e685D4A43181e6061;
    address internal constant ENA_OPTIMISM = 0x58538e6A46E07434d7E7375Bc268D3cb839C0133;
    address internal constant ENA_ARBITRUM = 0x58538e6A46E07434d7E7375Bc268D3cb839C0133;
    address internal constant ENA_BASE = 0x58538e6A46E07434d7E7375Bc268D3cb839C0133;

    // USDtb (BUIDL-backed) — 18 decimals. Ethereum + Arbitrum + Base only (NOT on Optimism).
    // The Ethereum ERC20 is an upgradeable, pausable proxy — factor pause/upgrade risk into guarantees.
    address internal constant USDTB_OFT_ETHEREUM = 0xc708B6887DB46005dA033501f8aeBee72d191a5d;
    address internal constant USDTB_TOKEN_ETHEREUM = 0xC139190F447e929f090Edeb554D95AbB8b18aC1C;
    address internal constant USDTB_ARBITRUM = 0xc708B6887DB46005dA033501f8aeBee72d191a5d;
    address internal constant USDTB_BASE = 0xc708B6887DB46005dA033501f8aeBee72d191a5d;

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
    uint32 internal constant EID_OPTIMISM = 30_111;
    uint32 internal constant EID_ARBITRUM = 30_110;
    uint32 internal constant EID_BASE = 30_184;
}
