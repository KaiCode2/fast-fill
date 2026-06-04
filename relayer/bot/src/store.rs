//! In-memory job store + lifecycle phases. Resets on restart (acceptable for a demo bot; the
//! watcher re-derives from chain head on startup).

use std::sync::Arc;
use std::time::Instant;

use alloy::primitives::{Address, B256, U256};
use dashmap::DashMap;

use crate::sol::Order;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Phase {
    Filling,
    Attesting, // CCTP: waiting on Circle attestation
    Settling,  // CCTP: settle tx in flight / OFT: awaiting LayerZero auto-delivery
    Settled,
    Failed,
    AttestTimeout,
}

impl Phase {
    pub fn is_terminal(self) -> bool {
        matches!(self, Phase::Settled | Phase::Failed | Phase::AttestTimeout)
    }
}

#[derive(Clone)]
#[allow(dead_code)] // some fields retained for provenance/diagnostics
pub struct Job {
    pub order_id: B256,
    pub order: Order,
    pub bridge_type: u8,
    pub oft_id: Option<u8>,
    pub adapter: Address,
    pub src_chain_id: u64,
    pub dst_chain_id: u64,
    pub src_tx_hash: B256,
    pub src_cctp_domain: u32,
    /// CCTP `mintFee` (USDC base units): `> 0` ⇒ executor-routed (settle via `CctpExecutor.execute`,
    /// which pays this fee to the relayer); `0` ⇒ direct (`CctpAdapter.settle`). OFT: 0.
    pub mint_fee: U256,
    pub phase: Phase,
    pub filled_by_us: bool,
    pub fill_tx: Option<B256>,
    pub settle_tx: Option<B256>,
    pub payout: Option<U256>,
    pub fee: Option<U256>,
    pub attempts: u32,
    pub next_action_at: Instant,
    pub created_at: Instant,
    pub inflight: bool,
    pub last_error: Option<String>,
}

#[derive(Clone, Default)]
pub struct JobStore {
    inner: Arc<DashMap<B256, Job>>,
}

impl JobStore {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn contains(&self, id: &B256) -> bool {
        self.inner.contains_key(id)
    }
    pub fn get(&self, id: &B256) -> Option<Job> {
        self.inner.get(id).map(|j| j.clone())
    }
    pub fn insert(&self, job: Job) {
        self.inner.insert(job.order_id, job);
    }
    pub fn ids(&self) -> Vec<B256> {
        self.inner.iter().map(|e| *e.key()).collect()
    }
    pub fn update<F: FnOnce(&mut Job)>(&self, id: &B256, f: F) {
        if let Some(mut j) = self.inner.get_mut(id) {
            f(&mut j);
        }
    }
}
