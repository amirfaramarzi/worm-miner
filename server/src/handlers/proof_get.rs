use std::sync::{Arc, RwLock};

use axum::{extract::State, response::IntoResponse};

use crate::data::AppState;

/// GET `/proof` returns minimum proving fee of the relayer.
pub async fn proof_get(State(state): State<Arc<RwLock<AppState>>>) -> ProofGetResponse {
    todo!();
}

pub struct ProofGetResponse {}

impl IntoResponse for ProofGetResponse {
    fn into_response(self) -> axum::response::Response {
        todo!()
    }
}
