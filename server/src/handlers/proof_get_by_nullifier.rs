use crate::{data::AppState, error::ServerError};
use axum::{
    extract::{Path, State},
    response::IntoResponse,
};
use std::sync::{Arc, RwLock};

///  GET `/proof/{nullifier}` gets cached proof for the given nullifier.
pub async fn proof_get_by_nullifier(
    State(state): State<Arc<RwLock<AppState>>>,
    Path(nullifier): Path<String>,
) -> Result<ProofGetByNullifierResponse, ServerError> {
    println!("nullifier: `{}`", nullifier);
    todo!();
}

pub struct ProofGetByNullifierResponse {}

impl IntoResponse for ProofGetByNullifierResponse {
    fn into_response(self) -> axum::response::Response {
        todo!()
    }
}
