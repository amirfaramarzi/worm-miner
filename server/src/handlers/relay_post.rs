use crate::{data::AppState, error::ServerError};
use axum::{Json, extract::State, response::IntoResponse};
use serde::Deserialize;
use std::sync::{Arc, RwLock};

/// POST `/relay` gets inputs of a `mintCoin()` transaction and submits on behalf of you.
pub async fn relay_post(
    State(state): State<Arc<RwLock<AppState>>>,
    Json(body): Json<RelayPostRequest>,
) -> Result<RelayPostResponse, ServerError> {
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
