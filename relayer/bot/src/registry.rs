//! On-chain bootstrap: discover deployed OFT adapters and resolve every chain's USDC + per-OFT
//! token (and its decimals) from `FastFillConfig` and `OftAdapterFactory`. Avoids hardcoding the
//! per-chain/per-OFT token matrix and stays correct if more OFTs are deployed later.

use std::collections::HashMap;

use alloy::primitives::{Address, U256};
use eyre::{Result, WrapErr};

use crate::chains::Chains;
use crate::config::{self, BRIDGE_CCTP, BRIDGE_OFT};
use crate::sol::{cfg, factory, IERC20};

#[derive(Clone)]
#[allow(dead_code)] // symbol retained for diagnostics
pub struct AdapterMeta {
    pub bridge_type: u8,
    pub oft_id: Option<u8>,
    pub symbol: String,
}

#[derive(Clone, Copy)]
#[allow(dead_code)] // decimals retained for future per-decimal cap logic
pub struct TokenInfo {
    pub token: Address,
    pub decimals: u8,
}

#[derive(Default)]
pub struct Registry {
    /// Addresses to watch for `OrderCreated` (CCTP adapter + every deployed OFT adapter).
    pub watch_addresses: Vec<Address>,
    /// Emitting adapter address → metadata. Adapter addresses are CREATE2-identical across chains.
    pub adapter_meta: HashMap<Address, AdapterMeta>,
    /// chain → USDC token.
    pub usdc: HashMap<u64, Address>,
    /// (chain, oftId) → token + decimals.
    pub oft_token: HashMap<(u64, u8), TokenInfo>,
    /// oftId → adapter address.
    pub oft_adapter: HashMap<u8, Address>,
}

impl Registry {
    /// Resolve the token (and decimals) delivered on `chain_id` for a given bridge family.
    pub fn output_token(
        &self,
        chain_id: u64,
        bridge_type: u8,
        oft_id: Option<u8>,
    ) -> Option<TokenInfo> {
        if bridge_type == BRIDGE_CCTP {
            self.usdc.get(&chain_id).map(|&t| TokenInfo {
                token: t,
                decimals: 6,
            })
        } else {
            oft_id.and_then(|id| self.oft_token.get(&(chain_id, id)).copied())
        }
    }
}

/// Retry a (re-buildable) RPC call a few times — tolerates transient RPC blips / rate limits at
/// startup. `$build` is re-evaluated each attempt, so pass the call builder expression, not a future.
macro_rules! retry {
    ($label:expr, $build:expr) => {{
        let mut out = None;
        for attempt in 1..=5u32 {
            match $build.await {
                Ok(v) => {
                    out = Some(Ok(v));
                    break;
                }
                Err(e) => {
                    if attempt == 5 {
                        out = Some(Err(eyre::Report::from(e)).wrap_err(format!(
                            "bootstrap {} failed after retries",
                            $label
                        )));
                    } else {
                        tracing::debug!(label = $label, attempt, error = %e, "bootstrap RPC failed; retrying");
                        tokio::time::sleep(std::time::Duration::from_millis(400u64 * attempt as u64)).await;
                    }
                }
            }
        }
        out.unwrap()
    }};
}

pub async fn bootstrap(chains: &Chains) -> Result<Registry> {
    let mut reg = Registry::default();

    // CCTP adapter is always watched.
    reg.watch_addresses.push(config::CCTP_ADAPTER);
    reg.adapter_meta.insert(
        config::CCTP_ADAPTER,
        AdapterMeta {
            bridge_type: BRIDGE_CCTP,
            oft_id: None,
            symbol: "USDC".into(),
        },
    );

    // Discover deployed OFT adapters (deterministic addresses are identical across chains).
    let any = chains.any_provider();
    let fac = factory::new(config::OFT_FACTORY, any);
    for oft_id in 0..=config::MAX_OFT_ID {
        let addr = retry!("adapterFor", fac.adapterFor(oft_id).call())?;
        let deployed = retry!("isDeployed", fac.isDeployed(oft_id).call())?;
        if !deployed {
            tracing::info!(
                oft_id,
                symbol = config::oft_symbol(oft_id),
                "OFT adapter not deployed; skipping"
            );
            continue;
        }
        reg.oft_adapter.insert(oft_id, addr);
        reg.watch_addresses.push(addr);
        reg.adapter_meta.insert(
            addr,
            AdapterMeta {
                bridge_type: BRIDGE_OFT,
                oft_id: Some(oft_id),
                symbol: config::oft_symbol(oft_id).into(),
            },
        );
    }

    // Per-chain token resolution.
    for c in config::CHAINS {
        let provider = chains.provider(c.chain_id)?;
        let conf = cfg::new(config::FASTFILL_CONFIG, provider.clone());

        let cc = retry!("chainConfig", conf.chainConfig(U256::from(c.chain_id)).call())?;
        if cc.usdc != Address::ZERO {
            reg.usdc.insert(c.chain_id, cc.usdc);
        }

        for &oft_id in reg.oft_adapter.keys() {
            let d = retry!("oftConfig", conf.oftConfig(U256::from(c.chain_id), oft_id).call())?;
            if d.token == Address::ZERO {
                continue;
            }
            let decimals =
                retry!("decimals", IERC20::new(d.token, provider.clone()).decimals().call())
                    .unwrap_or(18);
            reg.oft_token.insert(
                (c.chain_id, oft_id),
                TokenInfo {
                    token: d.token,
                    decimals,
                },
            );
        }
    }

    Ok(reg)
}
