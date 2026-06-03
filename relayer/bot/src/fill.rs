//! Fill stage: idempotency check → inventory → quote → policy gates → simulate → submit.
//! Port of `demo/src/lib/server/fill.ts`. The relayer is the filler and the payer.

use alloy::primitives::{Address, B256, U256};
use alloy::providers::DynProvider;
use eyre::{eyre, Result};

use crate::app::App;
use crate::config::BRIDGE_CCTP;
use crate::sol::{cctp, oft, to_oft_order};
use crate::verify::VerifiedOrder;

pub enum FillOutcome {
    Filled {
        tx_hash: B256,
        payout: U256,
        fee: U256,
    },
    WouldFill {
        payout: U256,
        fee: U256,
    }, // dry-run
    AlreadyActive,
    InsufficientInventory {
        have: U256,
        need: U256,
    },
    Skipped(String),
    Error(String),
}

fn now_secs() -> u64 {
    use std::time::{SystemTime, UNIX_EPOCH};
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs()
}

fn clip(s: String) -> String {
    s.chars().take(200).collect()
}

pub async fn fill_order(app: &App, v: &VerifiedOrder) -> FillOutcome {
    match try_fill(app, v).await {
        Ok(o) => o,
        Err(e) => FillOutcome::Error(clip(format!("{e}"))),
    }
}

async fn try_fill(app: &App, v: &VerifiedOrder) -> Result<FillOutcome> {
    let dst = v.dst_chain_id;
    let provider = app.chains.provider(dst)?;

    let out = app
        .registry
        .output_token(dst, v.bridge_type, v.oft_id)
        .ok_or_else(|| eyre!("no output token for dst chain {dst}"))?;
    let symbol = if v.bridge_type == BRIDGE_CCTP {
        "USDC".to_string()
    } else {
        crate::config::oft_symbol(v.oft_id.unwrap_or(u8::MAX)).to_string()
    };

    // Policy: token enabled + per-token size cap.
    let pol = app.settings.policy_for(&symbol);
    if !pol.enabled {
        return Ok(FillOutcome::Skipped(format!("token {symbol} not enabled")));
    }
    if v.order.outputAmount > pol.max_size {
        return Ok(FillOutcome::Skipped(format!(
            "outputAmount {} exceeds max {} for {symbol}",
            v.order.outputAmount, pol.max_size
        )));
    }

    let relayer = app.chains.require_relayer()?;

    // Inventory: relayer fronts the worst-case payout (outputAmount), reimbursed at settle.
    let balance = crate::sol::IERC20::new(out.token, provider.clone())
        .balanceOf(relayer)
        .call()
        .await?;
    if balance < v.order.outputAmount {
        return Ok(FillOutcome::InsufficientInventory {
            have: balance,
            need: v.order.outputAmount,
        });
    }

    // Idempotency: already filled/settled?
    if read_status(&provider, v).await? >= 1 {
        return Ok(FillOutcome::AlreadyActive);
    }

    // Profitability.
    let (payout, fee) = quote_fill(&provider, v, U256::from(now_secs())).await?;
    if fee < app.settings.min_fee {
        return Ok(FillOutcome::Skipped(format!(
            "fee {fee} below min {}",
            app.settings.min_fee
        )));
    }

    if app.settings.dry_run {
        return Ok(FillOutcome::WouldFill { payout, fee });
    }

    // Submit under the per-chain lock (simulate → send → receipt all inside).
    let lock = app.chains.lock(dst);
    let _guard = lock.lock().await;

    if read_status(&provider, v).await? >= 1 {
        return Ok(FillOutcome::AlreadyActive);
    }

    match submit_fill(&provider, v).await {
        Ok(tx_hash) => Ok(FillOutcome::Filled {
            tx_hash,
            payout,
            fee,
        }),
        Err(e) => {
            // Re-read on revert: another relayer may have filled between check and send.
            if read_status(&provider, v).await.unwrap_or(0) >= 1 {
                Ok(FillOutcome::AlreadyActive)
            } else {
                Ok(FillOutcome::Error(clip(format!("{e}"))))
            }
        }
    }
}

/// Read the destination `OrderRecord.status` (0 None, 1 Filled, 2 Settled).
pub async fn read_status(provider: &DynProvider, v: &VerifiedOrder) -> Result<u8> {
    let status = if v.bridge_type == BRIDGE_CCTP {
        cctp::new(v.adapter, provider.clone())
            .getOrder(v.order_id)
            .call()
            .await?
            .status
    } else {
        oft::new(v.adapter, provider.clone())
            .getOrder(v.order_id)
            .call()
            .await?
            .status
    };
    Ok(status)
}

async fn quote_fill(provider: &DynProvider, v: &VerifiedOrder, t: U256) -> Result<(U256, U256)> {
    if v.bridge_type == BRIDGE_CCTP {
        let q = cctp::new(v.adapter, provider.clone())
            .quoteFill(v.order.clone(), t)
            .call()
            .await?;
        Ok((q.payoutToRecipient, q.feeToFiller))
    } else {
        let q = oft::new(v.adapter, provider.clone())
            .quoteFill(to_oft_order(&v.order), t)
            .call()
            .await?;
        Ok((q.payoutToRecipient, q.feeToFiller))
    }
}

async fn submit_fill(provider: &DynProvider, v: &VerifiedOrder) -> Result<B256> {
    if v.bridge_type == BRIDGE_CCTP {
        let inst = cctp::new(v.adapter, provider.clone());
        inst.fill(v.order.clone()).call().await?; // simulate
        let pending = inst.fill(v.order.clone()).send().await?;
        Ok(pending.get_receipt().await?.transaction_hash)
    } else {
        let inst = oft::new(v.adapter, provider.clone());
        let order = to_oft_order(&v.order);
        inst.fill(order.clone()).call().await?; // simulate
        let pending = inst.fill(order).send().await?;
        Ok(pending.get_receipt().await?.transaction_hash)
    }
}

/// Used by the orchestrator's settle stage to detect terminal settlement.
pub async fn read_status_for(
    app: &App,
    adapter: Address,
    bridge_type: u8,
    dst_chain_id: u64,
    order_id: B256,
) -> Result<u8> {
    let provider = app.chains.provider(dst_chain_id)?;
    let status = if bridge_type == BRIDGE_CCTP {
        cctp::new(adapter, provider.clone())
            .getOrder(order_id)
            .call()
            .await?
            .status
    } else {
        oft::new(adapter, provider.clone())
            .getOrder(order_id)
            .call()
            .await?
            .status
    };
    Ok(status)
}
