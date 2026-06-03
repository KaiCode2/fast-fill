//! Autonomous fast-fill relayer bot.
//!
//! Watches `OrderCreated` on every source chain, reconstructs + verifies each order, fills the
//! profitable ones it can cover, and (for CCTP, when the forwarding service is disabled) relays the
//! Circle attestation on-chain via `settle` to be reimbursed. OFT settlement is auto-delivered by
//! LayerZero. Live by default; pass `--dry-run` to detect/quote/simulate without sending txs.

mod app;
mod cctp;
mod chains;
mod config;
mod fill;
mod orchestrator;
mod order;
mod registry;
mod sol;
mod store;
mod verify;
mod watcher;

use std::sync::Arc;

use eyre::{eyre, Result};
use tracing_subscriber::EnvFilter;

use app::App;
use store::JobStore;

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(
            EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info")),
        )
        .init();

    let dry_run_flag = std::env::args().any(|a| a == "--dry-run");
    let settings = config::load(dry_run_flag)?;

    if settings.dry_run {
        tracing::warn!("DRY-RUN mode — no transactions will be sent");
    } else if settings.private_key.is_none() {
        return Err(eyre!(
            "RELAYER_PRIVATE_KEY is required for live mode (or pass --dry-run)"
        ));
    }

    let chains = chains::Chains::build(&settings)?;
    match chains.relayer() {
        Some(r) => tracing::info!(relayer = %r, "relayer account"),
        None => tracing::info!("read-only (no signer): dry-run only"),
    }

    tracing::info!("bootstrapping registry from chain…");
    let registry = registry::bootstrap(&chains).await?;
    tracing::info!(
        watch_addresses = registry.watch_addresses.len(),
        oft_adapters = registry.oft_adapter.len(),
        "registry ready"
    );

    if !settings.dry_run {
        tracing::info!("ensuring token approvals…");
        chains::ensure_approvals(&chains, &registry, &settings).await?;
    }

    let app = Arc::new(App {
        settings,
        chains,
        registry,
        store: JobStore::new(),
    });

    let (tx, rx) = tokio::sync::mpsc::unbounded_channel();
    for c in config::CHAINS {
        let app2 = app.clone();
        let tx2 = tx.clone();
        tokio::spawn(async move { watcher::run_watcher(app2, c.chain_id, tx2).await });
    }
    drop(tx); // watchers hold their clones; the original isn't needed

    tracing::info!("relayer running");
    orchestrator::run(app, rx).await;
    Ok(())
}
