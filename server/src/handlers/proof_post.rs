use std::sync::{Arc, RwLock};

use axum::{Json, extract::State, response::IntoResponse};
use serde::Deserialize;

use crate::data::AppState;

/// POST `/proof` gets inputs of the proof-of-burn zk circuit and starts proving.
pub async fn proof_post(
    State(state): State<Arc<RwLock<AppState>>>,
    Json(body): Json<ProofPostRequest>,
) -> ProofPostResponse {
    todo!();
}

#[derive(Deserialize)]
pub struct ProofPostRequest {}

pub struct ProofPostResponse {}

impl IntoResponse for ProofPostResponse {
    fn into_response(self) -> axum::response::Response {
        todo!()
    }
}
