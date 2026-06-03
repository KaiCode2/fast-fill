//! CCTP settlement: poll Circle's Iris v2 attestation API and relay the mint on-chain via
//! `CctpAdapter.settle(message, attestation)`. Port of `demo/src/lib/server/cctp.ts`.
//!
//! With the forwarding service disabled, USDC is NOT auto-delivered on the destination: the bot
//! must fetch the attestation and submit `settle` itself to mint and be reimbursed for its fill.

use alloy::primitives::{Address, Bytes, B256};
use eyre::{Result, WrapErr};
use serde::Deserialize;

use crate::chains::Chains;
use crate::config::Settings;
use crate::sol::cctp;

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
