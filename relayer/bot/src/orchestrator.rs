//! Orchestrator: drives jobs through the lifecycle. On a discovered order it verifies and attempts
//! an optimistic fill; a periodic tick then drives CCTP settlement — `CctpAdapter.settle` for direct
//! orders we filled, or `CctpExecutor.execute` for `mintFee > 0` routed orders (the permissionless
//! mint-relay path, which we pursue whenever the `mintFee` covers gas, even for orders we did not
//! fill). OFT orders settle via the LayerZero executor automatically. Port of
//! `demo/src/lib/server/relayer.ts`, extended for the CCTP executor.

use std::sync::Arc;
use std::time::{Duration, Instant};

use alloy::primitives::U256;
use tokio::sync::mpsc::UnboundedReceiver;

use crate::app::App;
use crate::config::{self, BRIDGE_CCTP};
use crate::fill::{self, FillOutcome};
use crate::store::{Job, Phase};
use crate::verify::{reconstruct_and_verify, VerifiedOrder};
use crate::watcher::Discovered;
use crate::{cctp, fill as fillmod};

pub async fn run(app: Arc<App>, mut rx: UnboundedReceiver<Discovered>) {
    let mut ticker = tokio::time::interval(Duration::from_secs(app.settings.poll_interval_secs));
    loop {
        tokio::select! {
            maybe = rx.recv() => {
                match maybe {
                    Some(d) => {
                        if app.store.contains(&d.event.order_id) { continue; }
                        let app2 = app.clone();
                        tokio::spawn(async move { handle_new(app2, d).await; });
                    }
                    None => { /* all watchers gone; keep ticking */ }
                }
            }
            _ = ticker.tick() => {
                tick_settles(&app).await;
            }
        }
    }
}

/// CCTP order with `mintFee > 0` ⇒ settled via the executor; we may relay its mint for the fee.
fn is_routed(v_bridge_type: u8, mint_fee: U256) -> bool {
    v_bridge_type == BRIDGE_CCTP && mint_fee > U256::ZERO
}

async fn handle_new(app: Arc<App>, d: Discovered) {
    let v = match reconstruct_and_verify(&app, &d).await {
        Ok(v) => v,
        Err(e) => {
            tracing::warn!(order = %d.event.order_id, error = %e, "verification failed; skipping");
            return;
        }
    };
    if app.store.contains(&v.order_id) {
        return;
    }
    let src_domain = config::chain_def(v.src_chain_id)
        .map(|c| c.cctp_domain)
        .unwrap_or(0);
    let job = Job {
        order_id: v.order_id,
        order: v.order.clone(),
        bridge_type: v.bridge_type,
        oft_id: v.oft_id,
        adapter: v.adapter,
        src_chain_id: v.src_chain_id,
        dst_chain_id: v.dst_chain_id,
        src_tx_hash: v.src_tx_hash,
        src_cctp_domain: src_domain,
        mint_fee: v.mint_fee,
        phase: Phase::Filling,
        filled_by_us: false,
        fill_tx: None,
        settle_tx: None,
        payout: None,
        fee: None,
        attempts: 0,
        next_action_at: Instant::now(),
        created_at: Instant::now(),
        inflight: false,
        last_error: None,
    };
    app.store.insert(job);
    tracing::info!(
        order = %v.order_id, bridge = v.bridge_type, src = v.src_chain_id, dst = v.dst_chain_id,
        amount = %v.order.outputAmount, mint_fee = %v.mint_fee, "order verified"
    );
    run_fill_stage(&app, &v).await;
}

async fn run_fill_stage(app: &Arc<App>, v: &VerifiedOrder) {
    let id = v.order_id;
    let is_cctp = v.bridge_type == BRIDGE_CCTP;
    // A routed CCTP order is worth pursuing through settlement even if we don't fill it: relaying its
    // mint pays `mintFee`. Optimistic filling and mint relaying are independent profit sources.
    let pursue_relay = is_routed(v.bridge_type, v.mint_fee) && app.settings.mint_relay_enabled;
    let settle_phase = if is_cctp {
        Phase::Attesting
    } else {
        Phase::Settling
    };

    match fill::fill_order(app, v).await {
        FillOutcome::Filled {
            tx_hash,
            payout,
            fee,
        } => {
            app.store.update(&id, |j| {
                j.filled_by_us = true;
                j.fill_tx = Some(tx_hash);
                j.payout = Some(payout);
                j.fee = Some(fee);
                j.phase = settle_phase;
            });
            tracing::info!(order = %id, tx = %tx_hash, fee = %fee, "FILLED");
        }
        FillOutcome::AlreadyActive => {
            app.store.update(&id, |j| j.phase = settle_phase);
            tracing::info!(order = %id, "already filled (by us earlier or another relayer)");
        }
        FillOutcome::WouldFill { payout, fee } => {
            tracing::info!(order = %id, payout = %payout, fee = %fee, "[dry-run] would fill");
            // Still advance routed orders so the dry-run reports the mint-relay decision too.
            app.store.update(&id, |j| {
                j.phase = if pursue_relay {
                    Phase::Attesting
                } else {
                    Phase::Settled
                }
            });
        }
        FillOutcome::InsufficientInventory { have, need } => {
            tracing::warn!(order = %id, %have, %need, "insufficient inventory; not filling");
            advance_or_terminal(
                app,
                &id,
                pursue_relay,
                format!("insufficient inventory have={have} need={need}"),
            );
        }
        FillOutcome::Skipped(reason) => {
            tracing::info!(order = %id, %reason, "fill skipped");
            advance_or_terminal(app, &id, pursue_relay, reason);
        }
        FillOutcome::Error(e) => {
            tracing::warn!(order = %id, error = %e, "fill error");
            advance_or_terminal(app, &id, pursue_relay, e);
        }
    }
}

/// After a non-fill outcome, either keep the job alive to pursue mint relay, or mark it terminal.
fn advance_or_terminal(
    app: &Arc<App>,
    id: &alloy::primitives::B256,
    pursue_relay: bool,
    reason: String,
) {
    app.store.update(id, |j| {
        if pursue_relay {
            j.phase = Phase::Attesting; // not filled, but the mint relay may still be profitable
            j.last_error = Some(reason);
        } else {
            j.phase = Phase::Failed;
            j.last_error = Some(reason);
        }
    });
}

fn schedule_backoff(j: &mut Job) {
    j.attempts += 1;
    let secs = (5.0_f64 * 1.4_f64.powi(j.attempts as i32)) as u64;
    j.next_action_at = Instant::now() + Duration::from_secs(secs.clamp(4, 30));
}

async fn tick_settles(app: &Arc<App>) {
    for id in app.store.ids() {
        let job = match app.store.get(&id) {
            Some(j) => j,
            None => continue,
        };
        if job.phase.is_terminal() || job.inflight || Instant::now() < job.next_action_at {
            continue;
        }
        if !matches!(job.phase, Phase::Attesting | Phase::Settling) {
            continue;
        }
        if job.created_at.elapsed() > Duration::from_secs(app.settings.attest_timeout_secs) {
            app.store.update(&id, |j| {
                j.phase = Phase::AttestTimeout;
                j.last_error = Some("settlement not observed before timeout".into());
            });
            tracing::warn!(order = %id, "settlement timed out");
            continue;
        }
        app.store.update(&id, |j| j.inflight = true);
        let app2 = app.clone();
        tokio::spawn(async move { handle_settle(app2, job).await });
    }
}

async fn handle_settle(app: Arc<App>, job: Job) {
    let id = job.order_id;

    // Already settled on-chain (by us, the LZ executor, another relayer, or Circle)?
    if let Ok(2) =
        fillmod::read_status_for(&app, job.adapter, job.bridge_type, job.dst_chain_id, id).await
    {
        mark_settled(&app, &id, None, "observed on-chain");
        return;
    }

    // OFT settles via the LayerZero executor automatically — just keep watching.
    if job.bridge_type != BRIDGE_CCTP {
        app.store.update(&id, |j| {
            j.inflight = false;
            schedule_backoff(j);
        });
        return;
    }

    let routed = is_routed(job.bridge_type, job.mint_fee);

    // Direct orders are only worth settling if we filled them (to recover fronted capital); there is
    // no on-chain incentive otherwise.
    if !routed && !job.filled_by_us {
        app.store.update(&id, |j| {
            j.phase = Phase::Failed;
            j.last_error = Some("direct order not filled by us — nothing to settle".into());
            j.inflight = false;
        });
        return;
    }

    // Fetch the Circle attestation.
    let att =
        match cctp::poll_attestation(&app.settings, job.src_cctp_domain, job.src_tx_hash).await {
            Ok(a) => a,
            Err(e) => {
                tracing::warn!(order = %id, error = %e, "iris poll error");
                return backoff(&app, &id);
            }
        };
    if !att.is_ready() {
        tracing::debug!(order = %id, status = %att.status, "attestation pending");
        return backoff(&app, &id);
    }
    let message = att.message.unwrap();
    let attestation = att.attestation.unwrap();

    if app.settings.dry_run {
        if routed {
            tracing::info!(order = %id, mint_fee = %job.mint_fee, "[dry-run] would relay mint via CctpExecutor.execute");
        } else {
            tracing::info!(order = %id, "[dry-run] would settle via CctpAdapter.settle");
        }
        app.store.update(&id, |j| {
            j.phase = Phase::Settled;
            j.inflight = false;
        });
        return;
    }

    if routed {
        relay_mint(&app, &job, &message, &attestation).await;
    } else {
        settle_direct(&app, &job, &message, &attestation).await;
    }
}

/// Routed (`mintFee > 0`) path: relay the mint via the executor. Always relay orders we filled (to
/// recover our capital + earn `mintFee`); for orders we did not fill, relay only if `mintFee` covers
/// gas (pure mint-relay profit).
async fn relay_mint(app: &Arc<App>, job: &Job, message: &str, attestation: &str) {
    let id = job.order_id;

    if !job.filled_by_us {
        let relayer = match app.chains.require_relayer() {
            Ok(r) => r,
            Err(e) => {
                tracing::warn!(order = %id, error = %e, "no relayer key");
                return backoff(app, &id);
            }
        };
        match cctp::mint_relay_profitable(
            &app.chains,
            job.dst_chain_id,
            relayer,
            message,
            attestation,
            job.mint_fee,
            &app.settings,
        )
        .await
        {
            Ok((false, gas_cost)) => {
                tracing::info!(order = %id, mint_fee = %job.mint_fee, gas_cost_usdc = %gas_cost, "mint relay not profitable; skipping");
                app.store.update(&id, |j| {
                    j.phase = Phase::Failed;
                    j.last_error = Some(format!(
                        "mint relay unprofitable: mintFee {} < gas {}",
                        job.mint_fee, gas_cost
                    ));
                    j.inflight = false;
                });
                return;
            }
            Ok((true, gas_cost)) => {
                tracing::info!(order = %id, mint_fee = %job.mint_fee, gas_cost_usdc = %gas_cost, "mint relay profitable");
            }
            Err(e) => {
                // estimate failed (e.g. would revert / not yet relayable) — retry shortly.
                tracing::debug!(order = %id, error = %e, "mint relay gas estimate failed; retrying");
                return backoff(app, &id);
            }
        }
    }

    app.store.update(&id, |j| j.phase = Phase::Settling);
    match cctp::execute(&app.chains, job.dst_chain_id, message, attestation).await {
        Ok(tx) => {
            app.store.update(&id, |j| {
                j.settle_tx = Some(tx);
            });
            mark_settled(app, &id, Some(tx), "relayed mint (CctpExecutor.execute)");
        }
        Err(e) => settle_retry(app, job, e).await,
    }
}

/// Direct (`mintFee == 0`) path for an order we filled: `CctpAdapter.settle` to be reimbursed.
async fn settle_direct(app: &Arc<App>, job: &Job, message: &str, attestation: &str) {
    let id = job.order_id;
    app.store.update(&id, |j| j.phase = Phase::Settling);
    match cctp::settle(
        &app.chains,
        job.dst_chain_id,
        job.adapter,
        message,
        attestation,
    )
    .await
    {
        Ok(tx) => {
            app.store.update(&id, |j| {
                j.settle_tx = Some(tx);
            });
            mark_settled(app, &id, Some(tx), "settled (CctpAdapter.settle)");
        }
        Err(e) => settle_retry(app, job, e).await,
    }
}

/// A settle/execute tx failed: if it actually landed (raced by another relayer), accept it; else retry.
async fn settle_retry(app: &Arc<App>, job: &Job, e: eyre::Report) {
    let id = job.order_id;
    let settled = fillmod::read_status_for(app, job.adapter, job.bridge_type, job.dst_chain_id, id)
        .await
        .map(|s| s == 2)
        .unwrap_or(false);
    if settled {
        mark_settled(app, &id, None, "raced; settled by another party");
        return;
    }
    let msg: String = format!("{e}").chars().take(160).collect();
    tracing::warn!(order = %id, error = %msg, "settle/execute failed; will retry");
    app.store.update(&id, |j| {
        j.phase = Phase::Attesting;
        j.last_error = Some(msg);
        j.inflight = false;
        schedule_backoff(j);
    });
}

fn mark_settled(
    app: &Arc<App>,
    id: &alloy::primitives::B256,
    tx: Option<alloy::primitives::B256>,
    how: &str,
) {
    app.store.update(id, |j| {
        j.phase = Phase::Settled;
        j.inflight = false;
    });
    match tx {
        Some(tx) => tracing::info!(order = %id, %tx, "SETTLED ({how})"),
        None => tracing::info!(order = %id, "SETTLED ({how})"),
    }
}

fn backoff(app: &Arc<App>, id: &alloy::primitives::B256) {
    app.store.update(id, |j| {
        j.inflight = false;
        schedule_backoff(j);
    });
}
