//! Shared application state passed (as `Arc<App>`) to watchers and the orchestrator.

use crate::chains::Chains;
use crate::config::Settings;
use crate::registry::Registry;
use crate::store::JobStore;

pub struct App {
    pub settings: Settings,
    pub chains: Chains,
    pub registry: Registry,
    pub store: JobStore,
}
