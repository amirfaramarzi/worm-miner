pub mod config;

use crate::{
    data::config::Config,
    proof_queue_service::proof_job::{ProofJob, ProofResult},
};
use alloy::{
    consensus::Header,
    primitives::U256,
    providers::{Provider, RootProvider},
};
use std::{collections::HashMap, sync::Arc};
use tokio::sync::{RwLock, mpsc::UnboundedSender};

pub struct AppStateGeneric<P: Provider> {
    pub config: Config,
    pub provider: P,
    pub header_cache: HashMap<u64, Header>,
    /// <nullifier, ProofResult>
    pub proof_cache: HashMap<U256, ProofResult>,
    pub job_channel: UnboundedSender<ProofJob>,
}

pub type AppStateProvider = RootProvider;
pub type AppState = AppStateGeneric<AppStateProvider>;

impl AppState {
    pub fn new(
        config: Config,
        provider: AppStateProvider,
        job_channel: UnboundedSender<ProofJob>,
    ) -> Arc<RwLock<AppState>> {
        Arc::new(RwLock::new(AppState {
            config,
            provider,
            header_cache: Default::default(),
            proof_cache: Default::default(),
            job_channel,
        }))
    }
}
