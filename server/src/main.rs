mod data;
mod error;
mod handlers;
mod proof_queue_service;

use alloy::providers::RootProvider;
use axum::{
    Router,
    routing::{get, post},
};
use std::sync::Arc;
use tokio::sync::mpsc;

use crate::{
    data::{AppState, config::Config},
    handlers::*,
    proof_queue_service::{ProofQueueService, proof_job::ProofJob},
};

#[tokio::main]
async fn main() {
    let config = Config::load().expect("error file loading config file");
    let port = config.port;
    config.print();

    let provider = RootProvider::new_http("https://127.0.0.1:8545".try_into().unwrap());

    let (job_tx, job_rx) = mpsc::unbounded_channel::<ProofJob>();
    let state = AppState::new(config, provider, job_tx);

    ProofQueueService::new(job_rx, Arc::clone(&state)).start();

    let router = Router::new()
        .route("/proof", get(proof_get))
        .route("/proof", post(proof_post))
        .route("/proof/{nullifier}", get(proof_get_by_nullifier))
        .route("/relay", get(relay_get))
        .route("/relay", post(relay_post))
        .with_state(state);

    let address = format!("0.0.0.0:{}", port);
    let listener = tokio::net::TcpListener::bind(&address).await.unwrap();
    println!("Listening on `{}`", address);
    axum::serve(listener, router).await.unwrap();
}
