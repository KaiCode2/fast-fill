// SPDX-License-Identifier: MIT
pragma solidity ^0.8.26;

import {IFastFillConfig, ChainConfig, OftDeployment} from "../interfaces/IFastFillConfig.sol";
import {OftId} from "../libraries/OftId.sol";

/// @title  FastFillConfig
/// @notice The canonical, immutable chain registry for fast-fill. Every supported chain's bridge
///         addresses and transport ids are baked in as constants, so the bytecode is identical on
///         every chain and the contract can be CREATE2-deployed to one deterministic address
///         everywhere. Adapters take this single address and resolve all chain-specific data from
///         it — there are no owner setters and no per-deployment configuration to get wrong.
///
///         Transport config (CCTP + the chain's LZ eid) is keyed by chainId via `chainConfig`. Each
///         OFT's per-chain `(oft, token)` pair is keyed by `(chainId, oftId)` via `oftConfig`, so the
///         registry scales to many OFTs without growing the per-call `ChainConfig` struct. Onboarding
///         a new OFT is purely additive: assign it an `OftId`, add its rows here, and deploy one more
///         adapter via `OftAdapterFactory` — no new adapter code.
///
///         What is NOT baked is anything an adapter can read live on its own chain: the CCTP
///         MessageTransmitter (read from the TokenMessenger), the local CCTP domain (read from the
///         MessageTransmitter), and the OFT endpoint / token / eid (read from the OFT). Adapters
///         cross-check those live reads against this registry, so a wrong constant here cannot
///         silently ship — deployment/operation reverts on mismatch.
///
///         Sources: Circle CCTP v2 (developers.circle.com/cctp), LayerZero v2 deployments, USD₮0
///         (github.com/Everdawn-Labs/usdt0-audit-reports), and Ethena (docs.ethena.fi /
///         docs.usdtb.money). All values are additionally verified on-chain in the fork tests.
contract FastFillConfig is IFastFillConfig {
    // --- CCTP v2 TokenMessenger (CREATE2-identical across every supported chain) ---
    address internal constant CCTP_TOKEN_MESSENGER_V2 = 0x28b5a0e9C621a5BadaA536219b3a228C8168cf5d;

    // --- USDC (per chain) ---
    address internal constant USDC_ETHEREUM = 0xA0b86991c6218b36c1d19D4a2e9Eb0cE3606eB48;
    address internal constant USDC_OPTIMISM = 0x0b2C639c533813f4Aa9D7837CAf62653d097Ff85;
    address internal constant USDC_ARBITRUM = 0xaf88d065e77c8cC2239327C5EDb3A432268e5831;
    address internal constant USDC_BASE = 0x833589fCD6eDb6E08f4c7C32D4f71b54bdA02913;

    // --- LayerZero v2 endpoint ids (per chain; shared across all OFTs) ---
    uint32 internal constant EID_ETHEREUM = 30_101;
    uint32 internal constant EID_OPTIMISM = 30_111;
    uint32 internal constant EID_ARBITRUM = 30_110;
    uint32 internal constant EID_BASE = 30_184;

    // ---------------------------------------------------------------------------------------------
    // OFT deployments. For each OFT, the home chain (Ethereum) is an adapter/lockbox where the OFT
    // entrypoint and the ERC20 are DIFFERENT addresses; the L2s are native mint/burn OFTs where the
    // entrypoint IS the ERC20 (oft == token). A pair left at address(0) means "not on this chain".
    // ---------------------------------------------------------------------------------------------

    // --- USD₮0 (Ethereum is the lockbox OFTAdapter over USDT; Base has no USD₮0) ---
    address internal constant USDT0_OFT_ETHEREUM = 0x6C96dE32CEa08842dcc4058c14d3aaAD7Fa41dee;
    address internal constant USDT0_TOKEN_ETHEREUM = 0xdAC17F958D2ee523a2206206994597C13D831ec7; // USDT
    address internal constant USDT0_OFT_OPTIMISM = 0xF03b4d9AC1D5d1E7c4cEf54C2A313b9fe051A0aD;
    address internal constant USDT0_TOKEN_OPTIMISM = 0x01bFF41798a0BcF287b996046Ca68b395DbC1071;
    address internal constant USDT0_OFT_ARBITRUM = 0x14E4A1B13bf7F943c8ff7C51fb60FA964A298D92;
    address internal constant USDT0_TOKEN_ARBITRUM = 0xFd086bC7CD5C481DCC9C85ebE478A1C0b69FCbb9;

    // --- Ethena USDe (18 decimals; all four chains) ---
    address internal constant USDE_OFT_ETHEREUM = 0x5d3a1Ff2b6BAb83b63cd9AD0787074081a52ef34;
    address internal constant USDE_TOKEN_ETHEREUM = 0x4c9EDD5852cd905f086C759E8383e09bff1E68B3;
    address internal constant USDE_OPTIMISM = 0x5d3a1Ff2b6BAb83b63cd9AD0787074081a52ef34; // native: oft == token
    address internal constant USDE_ARBITRUM = 0x5d3a1Ff2b6BAb83b63cd9AD0787074081a52ef34;
    address internal constant USDE_BASE = 0x5d3a1Ff2b6BAb83b63cd9AD0787074081a52ef34;

    // --- Ethena Staked USDe / sUSDe (18 decimals; all four chains; ERC20 has a transfer blacklist) ---
    address internal constant SUSDE_OFT_ETHEREUM = 0x211Cc4DD073734dA055fbF44a2b4667d5E5fE5d2;
    address internal constant SUSDE_TOKEN_ETHEREUM = 0x9D39A5DE30e57443BfF2A8307A4256c8797A3497;
    address internal constant SUSDE_OPTIMISM = 0x211Cc4DD073734dA055fbF44a2b4667d5E5fE5d2;
    address internal constant SUSDE_ARBITRUM = 0x211Cc4DD073734dA055fbF44a2b4667d5E5fE5d2;
    address internal constant SUSDE_BASE = 0x211Cc4DD073734dA055fbF44a2b4667d5E5fE5d2;

    // --- Ethena ENA (18 decimals; all four chains; L2 OFT addr differs from the Ethereum adapter) ---
    address internal constant ENA_OFT_ETHEREUM = 0x90967a0Af8178ec6024Ab15461Ed79B8676071D0;
    address internal constant ENA_TOKEN_ETHEREUM = 0x57e114B691Db790C35207b2e685D4A43181e6061;
    address internal constant ENA_OPTIMISM = 0x58538e6A46E07434d7E7375Bc268D3cb839C0133;
    address internal constant ENA_ARBITRUM = 0x58538e6A46E07434d7E7375Bc268D3cb839C0133;
    address internal constant ENA_BASE = 0x58538e6A46E07434d7E7375Bc268D3cb839C0133;

    // --- Ethena USDtb (18 decimals; Ethereum + Arbitrum + Base only — NOT on Optimism) ---
    address internal constant USDTB_OFT_ETHEREUM = 0xc708B6887DB46005dA033501f8aeBee72d191a5d;
    address internal constant USDTB_TOKEN_ETHEREUM = 0xC139190F447e929f090Edeb554D95AbB8b18aC1C;
    address internal constant USDTB_ARBITRUM = 0xc708B6887DB46005dA033501f8aeBee72d191a5d;
    address internal constant USDTB_BASE = 0xc708B6887DB46005dA033501f8aeBee72d191a5d;

    /// @inheritdoc IFastFillConfig
    function chainConfig(uint256 chainId) external pure returns (ChainConfig memory) {
        if (chainId == 1) return ChainConfig(true, 0, EID_ETHEREUM, USDC_ETHEREUM, CCTP_TOKEN_MESSENGER_V2);
        if (chainId == 10) return ChainConfig(true, 2, EID_OPTIMISM, USDC_OPTIMISM, CCTP_TOKEN_MESSENGER_V2);
        if (chainId == 42_161) return ChainConfig(true, 3, EID_ARBITRUM, USDC_ARBITRUM, CCTP_TOKEN_MESSENGER_V2);
        if (chainId == 8453) return ChainConfig(true, 6, EID_BASE, USDC_BASE, CCTP_TOKEN_MESSENGER_V2);
        return ChainConfig(false, 0, 0, address(0), address(0));
    }

    /// @inheritdoc IFastFillConfig
    function oftConfig(uint256 chainId, uint8 oftId) external pure returns (OftDeployment memory) {
        if (chainId == 1) {
            if (oftId == OftId.USDT0) return OftDeployment(USDT0_OFT_ETHEREUM, USDT0_TOKEN_ETHEREUM);
            if (oftId == OftId.USDE) return OftDeployment(USDE_OFT_ETHEREUM, USDE_TOKEN_ETHEREUM);
            if (oftId == OftId.SUSDE) return OftDeployment(SUSDE_OFT_ETHEREUM, SUSDE_TOKEN_ETHEREUM);
            if (oftId == OftId.ENA) return OftDeployment(ENA_OFT_ETHEREUM, ENA_TOKEN_ETHEREUM);
            if (oftId == OftId.USDTB) return OftDeployment(USDTB_OFT_ETHEREUM, USDTB_TOKEN_ETHEREUM);
        } else if (chainId == 10) {
            if (oftId == OftId.USDT0) return OftDeployment(USDT0_OFT_OPTIMISM, USDT0_TOKEN_OPTIMISM);
            if (oftId == OftId.USDE) return OftDeployment(USDE_OPTIMISM, USDE_OPTIMISM);
            if (oftId == OftId.SUSDE) return OftDeployment(SUSDE_OPTIMISM, SUSDE_OPTIMISM);
            if (oftId == OftId.ENA) return OftDeployment(ENA_OPTIMISM, ENA_OPTIMISM);
            // USDtb: not deployed on Optimism.
        } else if (chainId == 42_161) {
            if (oftId == OftId.USDT0) return OftDeployment(USDT0_OFT_ARBITRUM, USDT0_TOKEN_ARBITRUM);
            if (oftId == OftId.USDE) return OftDeployment(USDE_ARBITRUM, USDE_ARBITRUM);
            if (oftId == OftId.SUSDE) return OftDeployment(SUSDE_ARBITRUM, SUSDE_ARBITRUM);
            if (oftId == OftId.ENA) return OftDeployment(ENA_ARBITRUM, ENA_ARBITRUM);
            if (oftId == OftId.USDTB) return OftDeployment(USDTB_ARBITRUM, USDTB_ARBITRUM);
        } else if (chainId == 8453) {
            // USD₮0: not deployed on Base.
            if (oftId == OftId.USDE) return OftDeployment(USDE_BASE, USDE_BASE);
            if (oftId == OftId.SUSDE) return OftDeployment(SUSDE_BASE, SUSDE_BASE);
            if (oftId == OftId.ENA) return OftDeployment(ENA_BASE, ENA_BASE);
            if (oftId == OftId.USDTB) return OftDeployment(USDTB_BASE, USDTB_BASE);
        }
        return OftDeployment(address(0), address(0));
    }
}
