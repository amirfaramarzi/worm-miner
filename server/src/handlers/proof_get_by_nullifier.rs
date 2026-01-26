use crate::{
    data::{AppState, Proof},
    error::ServerError,
};
use alloy::primitives::U256;
use anyhow::anyhow;
use axum::{
    Json,
    extract::{Path, State},
    response::IntoResponse,
};
use serde::Serialize;
use std::sync::Arc;
use tokio::sync::RwLock;

/// GET `/proof/{nullifier}` gets cached proof for the given nullifier.
/// nullifier should be base 10
pub async fn proof_get_by_nullifier(
    State(state): State<Arc<RwLock<AppState>>>,
    Path(nullifier): Path<String>,
) -> Result<ProofGetByNullifierResponse, ServerError> {
    let nullifier = U256::from_str_radix(&nullifier, 10)
        .map_err(|e| ServerError::validation("nullifier", nullifier.clone(), format!("{e}")))?;

    let state = state.read().await;

    match state.proof_cache.get(&nullifier) {
        Some(Ok(proof)) => Ok(ProofGetByNullifierResponse {
            proof: Some(proof.clone()),
            position_in_queue: None,
        }),
        Some(Err(err)) => Err(ServerError::Unexpected(
            anyhow!("{err}").into_boxed_dyn_error(),
        )),
        None => {
            if let Some(job_id) = state.nullifier_to_job_id.get(&nullifier) {
                Ok(ProofGetByNullifierResponse {
                    proof: None,
                    position_in_queue: state
                        .current_processing_job_id
                        .map(|curr| job_id.saturating_sub(curr)),
                })
            } else {
                Err(ServerError::NotFound("proof".to_string()))
            }
        }
    }
}

#[derive(Serialize)]
pub struct ProofGetByNullifierResponse {
    #[serde(flatten)]
    proof: Option<Proof>,
    position_in_queue: Option<usize>,
}

impl IntoResponse for ProofGetByNullifierResponse {
    fn into_response(self) -> axum::response::Response {
        Json(self).into_response()
    }
}
