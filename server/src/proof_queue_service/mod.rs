pub mod proof_job;

use crate::{
    data::{AppState, Proof},
    proof_queue_service::proof_job::ProofJob,
};
use alloy::primitives::U256;
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
            let block_number = job.header.number;
            let result = job.run(config.clone()).await;
            let result = result.map(|e| Proof::new(U256::from(block_number), e));
            state
                .write()
                .await
                .proof_cache
                .insert(job.nullifier, result);
        }
        println!("Channel closed");
    }
}
