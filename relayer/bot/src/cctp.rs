//! CCTP settlement + mint relay. Poll Circle's Iris v2 attestation API, then relay the mint on-chain.
//! Port of `demo/src/lib/server/cctp.ts`, plus the executor path.
//!
//! Two destination paths, selected at burn time by the order's `mintFee`:
//! - `mintFee == 0` (direct): `CctpAdapter.settle(message, attestation)` mints + settles.
//! - `mintFee  > 0` (routed): `CctpExecutor.execute(message, attestation)` mints, pays the relayer
//!   `mintFee` USDC (the permissionless drop-in for Circle's forwarding service), and forwards the
//!   rest to the adapter, which settles + reimburses any optimistic filler.

use alloy::primitives::{Address, Bytes, B256, U256};
use alloy::providers::Provider;
use eyre::{Result, WrapErr};
use serde::Deserialize;

use crate::chains::Chains;
use crate::config::{Settings, CCTP_EXECUTOR};
use crate::sol::{cctp, executor};

#[derive(Debug, Clone)]
pub struct Attestation {
    pub status: String,
    pub message: Option<String>,
    pub attestation: Option<String>,
}

impl Attestation {
    pub fn is_ready(&self) -> bool {
        self.status == "complete" && self.message.is_some() && self.attestation.is_some()
    }
}

#[derive(Deserialize)]
struct IrisResp {
    messages: Option<Vec<IrisMsg>>,
}

#[derive(Deserialize)]
struct IrisMsg {
    status: Option<String>,
    message: Option<String>,
    attestation: Option<String>,
}

/// Poll Circle's CCTP v2 attestation service for a burn tx.
pub async fn poll_attestation(
    settings: &Settings,
    src_domain: u32,
    burn_tx: B256,
) -> Result<Attestation> {
    let url = format!(
        "{}/v2/messages/{}?transactionHash={:#x}",
        settings.iris_base.trim_end_matches('/'),
        src_domain,
        burn_tx
    );
    let resp = reqwest::Client::new().get(&url).send().await?;
    let code = resp.status().as_u16();
    if code == 404 {
        return Ok(Attestation {
            status: "pending".into(),
            message: None,
            attestation: None,
        });
    }
    if !resp.status().is_success() {
        return Ok(Attestation {
            status: format!("http_{code}"),
            message: None,
            attestation: None,
        });
    }
    let body: IrisResp = resp.json().await?;
    let m = body
        .messages
        .and_then(|mut v| (!v.is_empty()).then(|| v.remove(0)));
    Ok(match m {
        None => Attestation {
            status: "pending".into(),
            message: None,
            attestation: None,
        },
        Some(m) => Attestation {
            status: m.status.unwrap_or_else(|| "pending".into()),
            message: m.message,
            attestation: m.attestation,
        },
    })
}

/// Relay the attested message on the destination: `settle` wraps `receiveMessage` → mint → reimburse.
pub async fn settle(
    chains: &Chains,
    dst_chain_id: u64,
    adapter: Address,
    message: &str,
    attestation: &str,
) -> Result<B256> {
    let provider = chains.provider(dst_chain_id)?;
    let msg: Bytes = message.parse().wrap_err("parse attestation message hex")?;
    let att: Bytes = attestation
        .parse()
        .wrap_err("parse attestation signature hex")?;
    let inst = cctp::new(adapter, provider.clone());

    let lock = chains.lock(dst_chain_id);
    let _guard = lock.lock().await;

    inst.settle(msg.clone(), att.clone()).call().await?; // simulate
    let pending = inst.settle(msg, att).send().await?;
    Ok(pending.get_receipt().await?.transaction_hash)
}

/// Relay a routed (`mintFee > 0`) order via the permissionless `CctpExecutor`: `receiveMessage`
/// mints USDC to the executor, which pays `mintFee` to `msg.sender` (us), forwards the remainder to
/// the adapter, and the adapter settles (reimbursing any filler). This is the mint-relay revenue path.
pub async fn execute(
    chains: &Chains,
    dst_chain_id: u64,
    message: &str,
    attestation: &str,
) -> Result<B256> {
    let provider = chains.provider(dst_chain_id)?;
    let msg: Bytes = message.parse().wrap_err("parse attestation message hex")?;
    let att: Bytes = attestation
        .parse()
        .wrap_err("parse attestation signature hex")?;
    let inst = executor::new(CCTP_EXECUTOR, provider.clone());

    let lock = chains.lock(dst_chain_id);
    let _guard = lock.lock().await;

    inst.execute(msg.clone(), att.clone()).call().await?; // simulate
    let pending = inst.execute(msg, att).send().await?;
    Ok(pending.get_receipt().await?.transaction_hash)
}

/// Is relaying this mint worth the gas? Estimates `execute` gas, values it in USDC via the configured
/// ETH price, and requires `mintFee` to cover it (and clear the absolute floor). Returns
/// `(profitable, gas_cost_usdc6)` for logging.
pub async fn mint_relay_profitable(
    chains: &Chains,
    dst_chain_id: u64,
    relayer: Address,
    message: &str,
    attestation: &str,
    mint_fee: U256,
    settings: &Settings,
) -> Result<(bool, U256)> {
    let provider = chains.provider(dst_chain_id)?;
    let msg: Bytes = message.parse().wrap_err("parse attestation message hex")?;
    let att: Bytes = attestation
        .parse()
        .wrap_err("parse attestation signature hex")?;
    let inst = executor::new(CCTP_EXECUTOR, provider.clone());

    let gas = inst.execute(msg, att).from(relayer).estimate_gas().await?;
    let gas_price = provider.get_gas_price().await?;
    let gas_cost = gas_cost_usdc6(gas, gas_price, settings.eth_price_usd);
    Ok((
        relay_is_profitable(mint_fee, gas_cost, settings.min_mint_fee),
        gas_cost,
    ))
}

/// Value a tx's gas in USDC base units (6-dp): `gas * gas_price(wei) * eth_price_usd / 1e12`.
fn gas_cost_usdc6(gas: u64, gas_price: u128, eth_price_usd: u64) -> U256 {
    U256::from(gas) * U256::from(gas_price) * U256::from(eth_price_usd)
        / U256::from(1_000_000_000_000u128)
}

/// Relay only if `mint_fee` covers the gas cost and clears the configured floor.
fn relay_is_profitable(mint_fee: U256, gas_cost_usdc6: U256, min_mint_fee: U256) -> bool {
    mint_fee >= gas_cost_usdc6 && mint_fee >= min_mint_fee
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn gas_cost_valuation_in_usdc6() {
        // 200_000 gas @ 0.001 gwei (1e6 wei) at $3000/ETH
        //   = 200_000 * 1e6 * 3000 / 1e12 = 600 base units (0.000600 USDC).
        assert_eq!(gas_cost_usdc6(200_000, 1_000_000, 3000), U256::from(600u64));
        // Zero gas price ⇒ zero cost.
        assert_eq!(gas_cost_usdc6(200_000, 0, 3000), U256::ZERO);
    }

    #[test]
    fn profitability_decision() {
        let cost = U256::from(600u64);
        // Covers gas, no floor ⇒ profitable.
        assert!(relay_is_profitable(U256::from(1000u64), cost, U256::ZERO));
        // Below gas cost ⇒ not profitable.
        assert!(!relay_is_profitable(U256::from(500u64), cost, U256::ZERO));
        // Covers gas but below the floor ⇒ not profitable.
        assert!(!relay_is_profitable(
            U256::from(1000u64),
            cost,
            U256::from(2000u64)
        ));
    }
}
