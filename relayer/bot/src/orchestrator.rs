//! Orchestrator: drives jobs through the lifecycle. On a discovered order it verifies + fills
//! immediately; a periodic tick advances filled CCTP orders through attestation → settle, and
//! watches OFT orders until LayerZero auto-settles. Port of `demo/src/lib/server/relayer.ts`.

use std::sync::Arc;
use std::time::{Duration, Instant};

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
        amount = %v.order.outputAmount, "order verified"
    );
    run_fill_stage(&app, &v).await;
}

async fn run_fill_stage(app: &Arc<App>, v: &VerifiedOrder) {
    let id = v.order_id;
    let is_cctp = v.bridge_type == BRIDGE_CCTP;
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
                j.phase = if is_cctp {
                    Phase::Attesting
                } else {
                    Phase::Settling
                };
            });
            tracing::info!(order = %id, tx = %tx_hash, fee = %fee, "FILLED");
        }
        FillOutcome::WouldFill { payout, fee } => {
            tracing::info!(order = %id, payout = %payout, fee = %fee, "[dry-run] would fill");
            app.store.update(&id, |j| j.phase = Phase::Settled);
        }
        FillOutcome::AlreadyActive => {
            app.store.update(&id, |j| {
                j.phase = if is_cctp {
                    Phase::Attesting
                } else {
                    Phase::Settling
                }
            });
            tracing::info!(order = %id, "already filled (by us earlier or another relayer)");
        }
        FillOutcome::InsufficientInventory { have, need } => {
            app.store.update(&id, |j| {
                j.phase = Phase::Failed;
                j.last_error = Some(format!("insufficient inventory have={have} need={need}"));
            });
            tracing::warn!(order = %id, %have, %need, "insufficient inventory; not filling");
        }
        FillOutcome::Skipped(reason) => {
            app.store.update(&id, |j| {
                j.phase = Phase::Failed;
                j.last_error = Some(reason.clone());
            });
            tracing::info!(order = %id, %reason, "skipped");
        }
        FillOutcome::Error(e) => {
            app.store.update(&id, |j| {
                j.phase = Phase::Failed;
                j.last_error = Some(e.clone());
            });
            tracing::warn!(order = %id, error = %e, "fill error");
        }
    }
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

    // First, has it already settled on-chain (by us, the LZ executor, or anyone)?
    if let Ok(2) =
        fillmod::read_status_for(&app, job.adapter, job.bridge_type, job.dst_chain_id, id).await
    {
        app.store.update(&id, |j| {
            j.phase = Phase::Settled;
            j.inflight = false;
        });
        tracing::info!(order = %id, "SETTLED (observed on-chain)");
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

    // CCTP: only self-settle orders we actually filled (to recover fronted capital).
    if !job.filled_by_us || app.settings.dry_run {
        app.store.update(&id, |j| {
            j.inflight = false;
            schedule_backoff(j);
        });
        return;
    }

    match cctp::poll_attestation(&app.settings, job.src_cctp_domain, job.src_tx_hash).await {
        Ok(att) if att.is_ready() => {
            app.store.update(&id, |j| j.phase = Phase::Settling);
            let message = att.message.unwrap();
            let attestation = att.attestation.unwrap();
            match cctp::settle(
                &app.chains,
                job.dst_chain_id,
                job.adapter,
                &message,
                &attestation,
            )
            .await
            {
                Ok(tx) => {
                    app.store.update(&id, |j| {
                        j.phase = Phase::Settled;
                        j.settle_tx = Some(tx);
                        j.inflight = false;
                    });
                    tracing::info!(order = %id, %tx, "SETTLED (relayed mint)");
                }
                Err(e) => {
                    let settled = fillmod::read_status_for(
                        &app,
                        job.adapter,
                        job.bridge_type,
                        job.dst_chain_id,
                        id,
                    )
                    .await
                    .map(|s| s == 2)
                    .unwrap_or(false);
                    if settled {
                        app.store.update(&id, |j| {
                            j.phase = Phase::Settled;
                            j.inflight = false;
                        });
                        tracing::info!(order = %id, "SETTLED (raced; settled by another party)");
                    } else {
                        let msg: String = format!("{e}").chars().take(160).collect();
                        tracing::warn!(order = %id, error = %msg, "settle failed; will retry");
                        app.store.update(&id, |j| {
                            j.phase = Phase::Attesting;
                            j.last_error = Some(msg);
                            j.inflight = false;
                            schedule_backoff(j);
                        });
                    }
                }
            }
        }
        Ok(att) => {
            tracing::debug!(order = %id, status = %att.status, "attestation pending");
            app.store.update(&id, |j| {
                j.inflight = false;
                schedule_backoff(j);
            });
        }
        Err(e) => {
            tracing::warn!(order = %id, error = %e, "iris poll error");
            app.store.update(&id, |j| {
                j.inflight = false;
                schedule_backoff(j);
            });
        }
    }
}
