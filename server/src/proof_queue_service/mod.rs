pub mod proof_job;

use crate::{data::AppState, proof_queue_service::proof_job::ProofJob};
use std::sync::Arc;
use tokio::sync::{RwLock, mpsc};

/// This runs ProofJobs one by one
pub struct ProofQueueService {
    jobs_channel: mpsc::UnboundedReceiver<ProofJob>,
    state: Arc<RwLock<AppState>>,
}

impl ProofQueueService {
    pub fn new(jobs: mpsc::UnboundedReceiver<ProofJob>, state: Arc<RwLock<AppState>>) -> Self {
        Self {
            jobs_channel: jobs,
            state,
        }
    }

    /// non blocking (fire and forget)
    pub fn start(self) {
        tokio::spawn(self.main_loop());
    }

    async fn main_loop(self) {
        let mut channel = self.jobs_channel;
        let state = self.state;
        let config = { state.read().await.config.clone() };

        while let Some(job) = channel.recv().await {
            let r = job.run(config.clone()).await;
            state.write().await.proof_cache.insert(job.nullifier, r);
        }
        println!("Channel closed");
    }
}
