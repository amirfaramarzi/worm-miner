use axum::{Json, response::IntoResponse};
use serde::Deserialize;

/// POST `/proof` gets inputs of the proof-of-burn zk circuit and starts proving.
pub async fn proof_post(Json(body): Json<ProofPostRequest>) -> ProofPostResponse {
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
