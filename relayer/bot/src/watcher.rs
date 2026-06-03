//! Per-chain `OrderCreated` watcher: polls `eth_getLogs` over a moving block cursor (with a
//! reorg-safety offset and ≤2000-block paging) and forwards discovered orders to the orchestrator.

use std::sync::Arc;
use std::time::Duration;

use alloy::primitives::{Address, B256, U256};
use alloy::providers::Provider;
use alloy::rpc::types::Filter;
use alloy::sol_types::SolEvent;
use eyre::Result;
use tokio::sync::mpsc::UnboundedSender;

use crate::app::App;
use crate::sol::cctp;

/// Decoded `OrderCreated` payload (the event carries only these 7 of the 16 `Order` fields).
#[derive(Clone, Debug)]
#[allow(dead_code)] // full event retained for debugging; not all fields drive logic
pub struct OrderCreatedData {
    pub order_id: B256,
    pub bridge_type: u8,
    pub sender: Address,
    pub dst_chain_id: u32,
    pub output_token: B256,
    pub output_amount: U256,
    pub nonce: u64,
}

#[derive(Clone, Debug)]
pub struct Discovered {
    pub src_chain_id: u64,
    pub adapter: Address,
    pub tx_hash: B256,
    pub block_number: u64,
    pub event: OrderCreatedData,
}

const MAX_BLOCK_SPAN: u64 = 2_000;

pub async fn run_watcher(app: Arc<App>, chain_id: u64, tx: UnboundedSender<Discovered>) {
    loop {
        if let Err(e) = watch_loop(&app, chain_id, &tx).await {
            tracing::warn!(chain_id, error = %e, "watcher loop error; restarting in 5s");
            tokio::time::sleep(Duration::from_secs(5)).await;
        }
    }
}

async fn watch_loop(app: &App, chain_id: u64, tx: &UnboundedSender<Discovered>) -> Result<()> {
    let provider = app.chains.provider(chain_id)?;
    let confirmations = app.settings.src_confirmations;
    let poll = Duration::from_secs(app.settings.poll_interval_secs);
    let addresses = app.registry.watch_addresses.clone();
    let topic = cctp::OrderCreated::SIGNATURE_HASH;

    let head = provider.get_block_number().await?;
    let mut cursor = head.saturating_sub(confirmations);
    tracing::info!(
        chain_id,
        start_block = cursor,
        addresses = addresses.len(),
        "watcher started"
    );

    loop {
        let head = provider.get_block_number().await?;
        let to = head.saturating_sub(confirmations);
        if to <= cursor {
            tokio::time::sleep(poll).await;
            continue;
        }

        let mut from = cursor + 1;
        while from <= to {
            let chunk_to = (from + MAX_BLOCK_SPAN - 1).min(to);
            let filter = Filter::new()
                .address(addresses.clone())
                .event_signature(topic)
                .from_block(from)
                .to_block(chunk_to);
            let logs = provider.get_logs(&filter).await?;
            for log in &logs {
                if let Some(d) = decode_discovered(chain_id, log) {
                    let _ = tx.send(d);
                }
            }
            from = chunk_to + 1;
        }
        cursor = to;
        tokio::time::sleep(poll).await;
    }
}

fn decode_discovered(chain_id: u64, log: &alloy::rpc::types::Log) -> Option<Discovered> {
    let decoded = cctp::OrderCreated::decode_log(&log.inner).ok()?;
    let e = decoded.data;
    Some(Discovered {
        src_chain_id: chain_id,
        adapter: log.inner.address,
        tx_hash: log.transaction_hash?,
        block_number: log.block_number?,
        event: OrderCreatedData {
            order_id: e.orderId,
            bridge_type: e.bridgeType,
            sender: e.sender,
            dst_chain_id: e.dstChainId,
            output_token: e.outputToken,
            output_amount: e.outputAmount,
            nonce: e.nonce,
        },
    })
}
