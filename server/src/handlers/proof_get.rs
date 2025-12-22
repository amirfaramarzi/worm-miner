use crate::{data::AppState, error::ServerError};
use axum::{extract::State, response::IntoResponse};
use std::sync::Arc;
use tokio::sync::RwLock;

/// GET `/proof` returns minimum proving fee of the relayer.
pub async fn proof_get(
    State(state): State<Arc<RwLock<AppState>>>,
) -> Result<ProofGetResponse, ServerError> {
    todo!();
}

pub struct ProofGetResponse {}

impl IntoResponse for ProofGetResponse {
    fn into_response(self) -> axum::response::Response {
        todo!()
    }
}
