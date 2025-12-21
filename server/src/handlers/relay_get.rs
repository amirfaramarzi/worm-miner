use std::sync::{Arc, RwLock};

use axum::{extract::State, response::IntoResponse};

use crate::data::AppState;

/// GET `/relay` returns minimum broadcasting fee of the relayer
pub async fn relay_get(State(state): State<Arc<RwLock<AppState>>>) -> RelayGetResponse {
    todo!();
}

pub struct RelayGetResponse {}

impl IntoResponse for RelayGetResponse {
    fn into_response(self) -> axum::response::Response {
        todo!()
    }
}
