use axum::{Json, response::IntoResponse};
use serde::Deserialize;

/// POST `/relay` gets inputs of a `mintCoin()` transaction and submits on behalf of you.
pub async fn relay_post(Json(body): Json<RelayPostRequest>) -> RelayPostResponse {
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
