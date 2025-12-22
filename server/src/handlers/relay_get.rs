use crate::{data::AppState, error::ServerError};
use axum::{extract::State, response::IntoResponse};
use std::sync::{Arc, RwLock};

/// GET `/relay` returns minimum broadcasting fee of the relayer
pub async fn relay_get(
    State(state): State<Arc<RwLock<AppState>>>,
) -> Result<RelayGetResponse, ServerError> {
    todo!();
}

pub struct RelayGetResponse {}

impl IntoResponse for RelayGetResponse {
    fn into_response(self) -> axum::response::Response {
        todo!()
    }
}
