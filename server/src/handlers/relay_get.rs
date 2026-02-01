use crate::{data::AppState, error::ServerError};
use alloy::primitives::U256;
use axum::{Json, extract::State, response::IntoResponse};
use common::utils::ether_amount_serializer;
use serde::Serialize;
use std::sync::Arc;
use tokio::sync::RwLock;

/// GET `/relay` returns minimum broadcasting fee of the relayer
pub async fn relay_get(
    State(state): State<Arc<RwLock<AppState>>>,
) -> Result<RelayGetResponse, ServerError> {
    let min_broadcaster_fee = state.read().await.config.min_broadcaster_fee;

    Ok(RelayGetResponse {
        min_broadcaster_fee,
    })
}

#[derive(Serialize)]
pub struct RelayGetResponse {
    #[serde(with = "ether_amount_serializer")]
    min_broadcaster_fee: U256,
}

impl IntoResponse for RelayGetResponse {
    fn into_response(self) -> axum::response::Response {
        Json(self).into_response()
    }
}
