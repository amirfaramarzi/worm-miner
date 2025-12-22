use crate::{
    data::AppState,
    error::{LogIfError, ServerError},
};
use alloy::primitives::U256;
use anyhow::anyhow;
use axum::{Json, extract::State, response::IntoResponse};
use common::utils::ether_amount_serializer;
use serde::Serialize;
use std::sync::{Arc, RwLock};

/// GET `/relay` returns minimum broadcasting fee of the relayer
pub async fn relay_get(
    State(state): State<Arc<RwLock<AppState>>>,
) -> Result<RelayGetResponse, ServerError> {
    let min_broadcaster_fee = state
        .read()
        .map_err(|e| ServerError::Unexpected(anyhow!("{e}")))
        .log_with_context("get_min_broadcaster_fee")?
        .config
        .min_broadcaster_fee;

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
