pub mod config;

use crate::{data::config::Config, proof_queue_service::proof_job::ProofJob};
use alloy::{consensus::Header, primitives::U256};
use common::mint::proof_generator::RapidsnarkOutput;
use std::{collections::HashMap, sync::Arc};
use tokio::sync::{RwLock, mpsc::UnboundedSender};

pub struct AppState {
    pub config: Config,

    /// block header cache, prevents extra onChain calls
    pub header_cache: HashMap<u64, Header>,

    /// <nullifier, ProofResult>
    pub proof_cache: HashMap<U256, Result<RapidsnarkOutput, anyhow::Error>>,
    pub job_channel: UnboundedSender<ProofJob>,
}

impl AppState {
    pub fn new(config: Config, job_channel: UnboundedSender<ProofJob>) -> Arc<RwLock<AppState>> {
        Arc::new(RwLock::new(AppState {
            config,
            header_cache: Default::default(),
            proof_cache: Default::default(),
            job_channel,
        }))
    }
}
