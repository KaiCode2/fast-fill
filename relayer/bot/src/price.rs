//! Live ETH/USD price with a TTL cache, used to value gas in the mint-relay profitability check.
//!
//! Fetches from a public source (CoinGecko by default; override with `RELAYER_ETH_PRICE_URL`) and
//! caches the result for `RELAYER_ETH_PRICE_TTL_SECS` (default 4h). On any fetch failure it falls
//! back to the last good value, or the static `RELAYER_ETH_PRICE_USD` if nothing has been fetched.

use std::time::{Duration, Instant};

use alloy::primitives::U256;
use eyre::{eyre, Result};
use tokio::sync::Mutex;

pub const FAST_FILL_GAS: u64 = 59_495;
pub const CCTP_DIRECT_SETTLE_GAS: u64 = 83_897;
pub const GAS_BUFFER_BPS: u64 = 2_500;

const BUFFER_DENOMINATOR: u64 = 10_000;
const WEI_PER_ETH: u128 = 1_000_000_000_000_000_000;

fn ceil_div(n: U256, d: U256) -> U256 {
    if n == U256::ZERO {
        U256::ZERO
    } else {
        (n + d - U256::from(1u8)) / d
    }
}

fn pow10(decimals: u8) -> U256 {
    U256::from(10u64).pow(U256::from(decimals))
}

/// Value buffered gas in USD-pegged token base units.
pub fn buffered_gas_cost_units(
    gas: u64,
    gas_price: u128,
    eth_price_usd: u64,
    token_decimals: u8,
) -> U256 {
    if gas == 0 || gas_price == 0 || eth_price_usd == 0 {
        return U256::ZERO;
    }
    let numerator = U256::from(gas)
        * U256::from(gas_price)
        * U256::from(eth_price_usd)
        * pow10(token_decimals)
        * U256::from(BUFFER_DENOMINATOR + GAS_BUFFER_BPS);
    let denominator = U256::from(WEI_PER_ETH) * U256::from(BUFFER_DENOMINATOR);
    ceil_div(numerator, denominator)
}

/// Minimum fill premium before fixed `RELAYER_MIN_FEE`, priced from benchmark gas.
pub fn fill_fee_floor(
    direct_cctp_settle: bool,
    callback_gas_limit: u64,
    gas_price: u128,
    eth_price_usd: u64,
    token_decimals: u8,
) -> U256 {
    let gas = FAST_FILL_GAS
        .saturating_add(callback_gas_limit)
        .saturating_add(if direct_cctp_settle {
            CCTP_DIRECT_SETTLE_GAS
        } else {
            0
        });
    buffered_gas_cost_units(gas, gas_price, eth_price_usd, token_decimals)
}

pub struct EthPriceCache {
    client: reqwest::Client,
    url: String,
    fallback_usd: u64,
    ttl: Duration,
    cache: Mutex<Option<(u64, Instant)>>,
}

impl EthPriceCache {
    pub fn new(url: String, fallback_usd: u64, ttl: Duration) -> Self {
        let client = reqwest::Client::builder()
            .user_agent("fastfill-relayer/0.1")
            .timeout(Duration::from_secs(10))
            .build()
            .unwrap_or_else(|_| reqwest::Client::new());
        Self {
            client,
            url,
            fallback_usd,
            ttl,
            cache: Mutex::new(None),
        }
    }

    /// Current ETH price in whole USD. Returns the cached value while fresh; otherwise refetches.
    /// On fetch failure, falls back to the last good value, else the configured static fallback.
    pub async fn get(&self) -> u64 {
        {
            let guard = self.cache.lock().await;
            if let Some((price, at)) = *guard {
                if at.elapsed() < self.ttl {
                    return price;
                }
            }
        }
        match self.fetch().await {
            Ok(price) => {
                tracing::info!(eth_usd = price, "fetched ETH price");
                *self.cache.lock().await = Some((price, Instant::now()));
                price
            }
            Err(e) => {
                let prev = self.cache.lock().await.map(|(p, _)| p);
                let used = prev.unwrap_or(self.fallback_usd);
                tracing::warn!(error = %e, eth_usd = used, "ETH price fetch failed; using fallback");
                used
            }
        }
    }

    async fn fetch(&self) -> Result<u64> {
        let resp = self.client.get(&self.url).send().await?;
        if !resp.status().is_success() {
            return Err(eyre!("price http {}", resp.status().as_u16()));
        }
        let body: serde_json::Value = resp.json().await?;
        parse_eth_usd(&body).ok_or_else(|| eyre!("unexpected price response shape"))
    }
}

/// Parse CoinGecko's `{"ethereum":{"usd":<number>}}` into whole USD.
fn parse_eth_usd(v: &serde_json::Value) -> Option<u64> {
    let p = v.get("ethereum")?.get("usd")?.as_f64()?;
    if p > 0.0 {
        Some(p.round() as u64)
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_coingecko_shape() {
        let v = serde_json::json!({"ethereum": {"usd": 3456.78}});
        assert_eq!(parse_eth_usd(&v), Some(3457));
    }

    #[test]
    fn rejects_bad_shape() {
        assert_eq!(parse_eth_usd(&serde_json::json!({"foo": 1})), None);
        assert_eq!(
            parse_eth_usd(&serde_json::json!({"ethereum": {"usd": 0}})),
            None
        );
        assert_eq!(parse_eth_usd(&serde_json::json!({"ethereum": {}})), None);
    }

    #[test]
    fn buffered_gas_cost_ceil_math() {
        // 200_000 gas @ 0.001 gwei (1e6 wei), $3000/ETH, 6-dp token:
        // unbuffered = 600 units; +25% = 750 units.
        assert_eq!(
            buffered_gas_cost_units(200_000, 1_000_000, 3000, 6),
            U256::from(750u64)
        );
        // One wei of cost still rounds up to one token base unit.
        assert_eq!(buffered_gas_cost_units(1, 1, 1, 6), U256::from(1u64));
    }

    #[test]
    fn fill_fee_floor_modes() {
        let gp = 1_000_000;
        let eth = 3000;
        assert_eq!(
            fill_fee_floor(false, 0, gp, eth, 6),
            buffered_gas_cost_units(FAST_FILL_GAS, gp, eth, 6)
        );
        assert_eq!(
            fill_fee_floor(true, 0, gp, eth, 6),
            buffered_gas_cost_units(FAST_FILL_GAS + CCTP_DIRECT_SETTLE_GAS, gp, eth, 6)
        );
        assert_eq!(
            fill_fee_floor(false, 100_000, gp, eth, 6),
            buffered_gas_cost_units(FAST_FILL_GAS + 100_000, gp, eth, 6)
        );
    }
}
