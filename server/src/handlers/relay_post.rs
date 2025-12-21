use std::sync::{Arc, RwLock};

use axum::{Json, extract::State, response::IntoResponse};
use serde::Deserialize;

use crate::data::AppState;

/// POST `/relay` gets inputs of a `mintCoin()` transaction and submits on behalf of you.
pub async fn relay_post(
    State(state): State<Arc<RwLock<AppState>>>,
    Json(body): Json<RelayPostRequest>,
) -> RelayPostResponse {
    todo!();
}

#[derive(Deserialize)]
pub struct RelayPostRequest {}

pub struct RelayPostResponse {}

impl IntoResponse for RelayPostResponse {
    fn into_response(self) -> axum::response::Response {
        todo!()
    }
}
