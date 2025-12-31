use crate::{data::AppState, error::ServerError};
use alloy::primitives::U256;
use axum::{Json, extract::State, response::IntoResponse};
use common::utils::ether_amount_serializer;
use serde::Serialize;
use std::sync::Arc;
use tokio::sync::RwLock;

/// GET `/proof` returns minimum proving fee of the relayer.
pub async fn proof_get(
    State(state): State<Arc<RwLock<AppState>>>,
) -> Result<ProofGetResponse, ServerError> {
    let state = state.read().await;

    Ok(ProofGetResponse {
        min_prover_fee: state.config.min_prover_fee,
    })
}

#[derive(Serialize)]
pub struct ProofGetResponse {
    #[serde(with = "ether_amount_serializer")]
    min_prover_fee: U256,
}

impl IntoResponse for ProofGetResponse {
    fn into_response(self) -> axum::response::Response {
        Json(&self).into_response()
    }
}
