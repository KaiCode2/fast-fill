//! Per-chain providers/signers, per-chain tx serialization locks, and one-time token approvals.

use std::collections::HashMap;
use std::sync::Arc;

use alloy::network::EthereumWallet;
use alloy::primitives::{Address, U256};
use alloy::providers::{DynProvider, Provider, ProviderBuilder};
use alloy::signers::local::PrivateKeySigner;
use eyre::{eyre, Result, WrapErr};
use tokio::sync::Mutex;

use crate::config::{self, Settings};
use crate::registry::Registry;
use crate::sol::IERC20;

pub struct Chains {
    providers: HashMap<u64, DynProvider>,
    locks: HashMap<u64, Arc<Mutex<()>>>,
    relayer: Option<Address>,
}

impl Chains {
    pub fn build(settings: &Settings) -> Result<Self> {
        let signer: Option<PrivateKeySigner> = match &settings.private_key {
            Some(pk) => {
                let hex = pk.strip_prefix("0x").unwrap_or(pk);
                Some(hex.parse().wrap_err("invalid RELAYER_PRIVATE_KEY")?)
            }
            None => None,
        };
        let relayer = signer.as_ref().map(|s| s.address());

        let mut providers = HashMap::new();
        let mut locks = HashMap::new();
        for c in config::CHAINS {
            let raw = settings
                .rpc_urls
                .get(&c.chain_id)
                .ok_or_else(|| eyre!("no rpc for chain {}", c.chain_id))?;
            let url = reqwest::Url::parse(raw).wrap_err("bad rpc url")?;
            let provider = match &signer {
                Some(s) => {
                    let wallet = EthereumWallet::from(s.clone());
                    ProviderBuilder::new()
                        .wallet(wallet)
                        .connect_http(url)
                        .erased()
                }
                None => ProviderBuilder::new().connect_http(url).erased(),
            };
            providers.insert(c.chain_id, provider);
            locks.insert(c.chain_id, Arc::new(Mutex::new(())));
        }

        Ok(Self {
            providers,
            locks,
            relayer,
        })
    }

    pub fn provider(&self, chain_id: u64) -> Result<DynProvider> {
        self.providers
            .get(&chain_id)
            .cloned()
            .ok_or_else(|| eyre!("no provider for chain {}", chain_id))
    }

    pub fn any_provider(&self) -> DynProvider {
        self.providers
            .values()
            .next()
            .cloned()
            .expect("at least one provider")
    }

    pub fn relayer(&self) -> Option<Address> {
        self.relayer
    }

    pub fn require_relayer(&self) -> Result<Address> {
        self.relayer
            .ok_or_else(|| eyre!("no relayer key configured"))
    }

    /// Per-chain mutex. Hold it across the whole simulate→send→await-receipt section so the
    /// nonce filler always reads a settled nonce (no double-spend across concurrent jobs).
    pub fn lock(&self, chain_id: u64) -> Arc<Mutex<()>> {
        self.locks
            .get(&chain_id)
            .cloned()
            .expect("lock for known chain")
    }
}

/// Approve each adapter to pull the relayer's enabled tokens (idempotent: skips if already high).
pub async fn ensure_approvals(chains: &Chains, reg: &Registry, settings: &Settings) -> Result<()> {
    let relayer = chains.require_relayer()?;
    for c in config::CHAINS {
        let provider = chains.provider(c.chain_id)?;

        if settings.policy_for("USDC").enabled {
            if let Some(&usdc) = reg.usdc.get(&c.chain_id) {
                approve_if_needed(&provider, usdc, relayer, config::CCTP_ADAPTER).await?;
            }
        }
        for (&oft_id, &adapter) in reg.oft_adapter.iter() {
            if !settings.policy_for(config::oft_symbol(oft_id)).enabled {
                continue;
            }
            if let Some(info) = reg.oft_token.get(&(c.chain_id, oft_id)) {
                approve_if_needed(&provider, info.token, relayer, adapter).await?;
            }
        }
    }
    Ok(())
}

async fn approve_if_needed(
    provider: &DynProvider,
    token: Address,
    owner: Address,
    spender: Address,
) -> Result<()> {
    let erc = IERC20::new(token, provider.clone());
    let allowance = erc.allowance(owner, spender).call().await?;
    // Treat anything below ~1e30 base units as "needs (re)approval".
    let threshold = U256::from(10u64).pow(U256::from(30u64));
    if allowance >= threshold {
        return Ok(());
    }
    tracing::info!(%token, %spender, "approving adapter (max) for token");
    let pending = erc.approve(spender, U256::MAX).send().await?;
    let receipt = pending.get_receipt().await?;
    tracing::info!(%token, %spender, tx = %receipt.transaction_hash, "approved");
    Ok(())
}
