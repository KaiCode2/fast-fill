//! Static chain table, deployed contract addresses, and env-driven settings.

use std::collections::HashMap;
use std::sync::Arc;
use std::time::Duration;

use alloy::primitives::{address, Address, U256};
use eyre::{eyre, Result};

use crate::price::EthPriceCache;

pub const BRIDGE_CCTP: u8 = 0;
pub const BRIDGE_OFT: u8 = 1;

// Deployed mainnet contracts — CREATE2-identical on every chain. Source: DEPLOYMENTS.md.
pub const FASTFILL_CONFIG: Address = address!("aec766479DB174110958Bc45D141A2C5eF693DF5");
// Executor-enabled CctpAdapter (supports mintFee-routed orders). Replaces the pre-executor
// historical adapter 0xcceeB77d7E4FD660fFd8E501a29A58ec6133cF0E.
pub const CCTP_ADAPTER: Address = address!("9FA37faBfA1Fd31Afe5A5F93e1c4Cd986b27bA75");
// Permissionless CCTP mint-relay singleton — settles `mintFee > 0` orders and pays the relayer.
pub const CCTP_EXECUTOR: Address = address!("AFc7bBc0B5fD7A4d9b936349cfE991e5bC6E2a80");
pub const OFT_FACTORY: Address = address!("84Bb5d3142024da8d61CBEE0A4c722a1650fbFcb");

/// Per-order destination-execution gas cap enforced on-chain (`FastFillBase.MAX_CALLBACK_GAS_LIMIT`).
#[allow(dead_code)]
pub const MAX_CALLBACK_GAS_LIMIT: u64 = 5_000_000;

/// Highest OFT id we probe for at startup (`OftId`: 0..=4). Append-only on-chain.
pub const MAX_OFT_ID: u8 = 4;

#[derive(Clone, Copy)]
#[allow(dead_code)] // name/lz_eid/explorer retained for logging + future use
pub struct ChainDef {
    pub chain_id: u64,
    pub name: &'static str,
    pub cctp_domain: u32,
    pub lz_eid: u32,
    pub alchemy_slug: &'static str,
    pub explorer: &'static str,
}

pub const CHAINS: &[ChainDef] = &[
    ChainDef {
        chain_id: 42161,
        name: "Arbitrum",
        cctp_domain: 3,
        lz_eid: 30110,
        alchemy_slug: "arb-mainnet",
        explorer: "https://arbiscan.io",
    },
    ChainDef {
        chain_id: 10,
        name: "Optimism",
        cctp_domain: 2,
        lz_eid: 30111,
        alchemy_slug: "opt-mainnet",
        explorer: "https://optimistic.etherscan.io",
    },
    ChainDef {
        chain_id: 8453,
        name: "Base",
        cctp_domain: 6,
        lz_eid: 30184,
        alchemy_slug: "base-mainnet",
        explorer: "https://basescan.org",
    },
];

pub fn chain_def(chain_id: u64) -> Option<&'static ChainDef> {
    CHAINS.iter().find(|c| c.chain_id == chain_id)
}

/// Symbol for an OFT id (mirrors `src/libraries/OftId.sol`).
pub fn oft_symbol(oft_id: u8) -> &'static str {
    match oft_id {
        0 => "USDT0",
        1 => "USDe",
        2 => "sUSDe",
        3 => "ENA",
        4 => "USDtb",
        _ => "OFT?",
    }
}

#[derive(Clone)]
pub struct TokenPolicy {
    pub enabled: bool,
    pub max_size: U256, // in the token's own base units
}

#[derive(Clone)]
pub struct Settings {
    pub private_key: Option<String>,
    pub rpc_urls: HashMap<u64, String>,
    pub min_fee: U256,
    pub src_confirmations: u64,
    pub iris_base: String,
    pub dry_run: bool,
    pub poll_interval_secs: u64,
    pub attest_timeout_secs: u64,
    pub default_max_size: U256,
    /// Keyed by UPPERCASE symbol. Absent ⇒ disabled.
    pub token_policy: HashMap<String, TokenPolicy>,
    // --- CCTP mint relay (CctpExecutor) ---
    /// Relay `mintFee > 0` CCTP mints for the fee (independent of optimistic filling).
    pub mint_relay_enabled: bool,
    /// Absolute floor (USDC base units, 6-dp) on `mintFee` before we relay an order we did not fill.
    pub min_mint_fee: U256,
    /// Live (TTL-cached) ETH/USD price used to value gas when checking mint-relay profitability.
    pub eth_price: Arc<EthPriceCache>,
}

impl Settings {
    pub fn policy_for(&self, symbol: &str) -> TokenPolicy {
        self.token_policy
            .get(&symbol.to_uppercase())
            .cloned()
            .unwrap_or(TokenPolicy {
                enabled: false,
                max_size: self.default_max_size,
            })
    }
}

fn env_u128(key: &str, default: u128) -> u128 {
    std::env::var(key)
        .ok()
        .and_then(|s| s.parse().ok())
        .unwrap_or(default)
}

/// Load settings from the environment (and an optional `.env`). `dry_run` is the CLI flag value.
pub fn load(dry_run_flag: bool) -> Result<Settings> {
    let _ = dotenvy::dotenv();

    let private_key = std::env::var("RELAYER_PRIVATE_KEY")
        .ok()
        .filter(|s| !s.is_empty());
    let alchemy_key = std::env::var("ALCHEMY_API_KEY")
        .ok()
        .filter(|s| !s.is_empty());

    let mut rpc_urls = HashMap::new();
    for c in CHAINS {
        let explicit = std::env::var(format!("RPC_URL_{}", c.chain_id))
            .ok()
            .filter(|s| !s.is_empty());
        let url = if let Some(u) = explicit {
            u
        } else if let Some(k) = &alchemy_key {
            format!("https://{}.g.alchemy.com/v2/{}", c.alchemy_slug, k)
        } else {
            return Err(eyre!(
                "no RPC for chain {} — set ALCHEMY_API_KEY or RPC_URL_{}",
                c.chain_id,
                c.chain_id
            ));
        };
        rpc_urls.insert(c.chain_id, url);
    }

    let default_max_size = U256::from(env_u128("RELAYER_MAX_BASE_UNITS", 5_000_000));
    let min_fee = U256::from(env_u128("RELAYER_MIN_FEE", 0));
    let src_confirmations = env_u128("RELAYER_SRC_CONFIRMATIONS", 1) as u64;
    let iris_base =
        std::env::var("CIRCLE_IRIS_BASE").unwrap_or_else(|_| "https://iris-api.circle.com".into());

    // Default: USDC + USDT0 enabled (both 6-dp); other OFTs (18-dp) off unless explicitly enabled.
    let enabled: Vec<String> = match std::env::var("RELAYER_ENABLED_TOKENS") {
        Ok(list) => list
            .split(',')
            .map(|s| s.trim().to_uppercase())
            .filter(|s| !s.is_empty())
            .collect(),
        Err(_) => vec!["USDC".into(), "USDT0".into()],
    };
    let mut token_policy = HashMap::new();
    for sym in enabled {
        let max_size = std::env::var(format!("RELAYER_MAX_{sym}"))
            .ok()
            .and_then(|s| s.parse::<u128>().ok())
            .map(U256::from)
            .unwrap_or(default_max_size);
        token_policy.insert(
            sym,
            TokenPolicy {
                enabled: true,
                max_size,
            },
        );
    }

    let dry_run = dry_run_flag || std::env::var("RELAYER_DRY_RUN").ok().as_deref() == Some("1");

    // Mint relay defaults on; disable with RELAYER_MINT_RELAY=0.
    let mint_relay_enabled = std::env::var("RELAYER_MINT_RELAY").ok().as_deref() != Some("0");
    let min_mint_fee = U256::from(env_u128("RELAYER_MIN_MINT_FEE", 0));

    // Live ETH price (cached) to value gas in the profitability check. RELAYER_ETH_PRICE_USD is the
    // fallback if the fetch fails; RELAYER_ETH_PRICE_TTL_SECS controls cache freshness (default 4h).
    let eth_price_fallback = env_u128("RELAYER_ETH_PRICE_USD", 3000) as u64;
    let eth_price_ttl =
        Duration::from_secs(env_u128("RELAYER_ETH_PRICE_TTL_SECS", 4 * 3600) as u64);
    let eth_price_url = std::env::var("RELAYER_ETH_PRICE_URL").unwrap_or_else(|_| {
        "https://api.coingecko.com/api/v3/simple/price?ids=ethereum&vs_currencies=usd".into()
    });
    let eth_price = Arc::new(EthPriceCache::new(
        eth_price_url,
        eth_price_fallback,
        eth_price_ttl,
    ));

    Ok(Settings {
        private_key,
        rpc_urls,
        min_fee,
        src_confirmations,
        iris_base,
        dry_run,
        poll_interval_secs: env_u128("RELAYER_POLL_SECS", 4) as u64,
        attest_timeout_secs: env_u128("RELAYER_ATTEST_TIMEOUT_SECS", 30 * 60) as u64,
        default_max_size,
        token_policy,
        mint_relay_enabled,
        min_mint_fee,
        eth_price,
    })
}
