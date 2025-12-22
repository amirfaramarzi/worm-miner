use crate::{
    data::AppState,
    error::{LogIfError, ServerError},
};
use alloy::primitives::U256;
use anyhow::anyhow;
use axum::{Json, extract::State, response::IntoResponse};
use common::utils::ether_amount_serializer;
use serde::Serialize;
use std::sync::{Arc, RwLock};

/// GET `/proof` returns minimum proving fee of the relayer.
pub async fn proof_get(
    State(state): State<Arc<RwLock<AppState>>>,
) -> Result<ProofGetResponse, ServerError> {
    let min_prover_fee = state
        .read()
        .map_err(|e| ServerError::Unexpected(anyhow!("{e}")))
        .log_with_context("get_min_prover_fee")?
        .config
        .min_prover_fee;

    Ok(ProofGetResponse { min_prover_fee })
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
