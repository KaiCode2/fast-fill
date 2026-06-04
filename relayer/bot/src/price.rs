//! Live ETH/USD price with a TTL cache, used to value gas in the mint-relay profitability check.
//!
//! Fetches from a public source (CoinGecko by default; override with `RELAYER_ETH_PRICE_URL`) and
//! caches the result for `RELAYER_ETH_PRICE_TTL_SECS` (default 4h). On any fetch failure it falls
//! back to the last good value, or the static `RELAYER_ETH_PRICE_USD` if nothing has been fetched.

use std::time::{Duration, Instant};

use eyre::{eyre, Result};
use tokio::sync::Mutex;

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
}
