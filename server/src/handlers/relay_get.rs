use axum::response::IntoResponse;

/// GET `/relay` returns minimum broadcasting fee of the relayer
pub async fn relay_get() -> RelayGetResponse {
    todo!();
}

pub struct RelayGetResponse {}

impl IntoResponse for RelayGetResponse {
    fn into_response(self) -> axum::response::Response {
        todo!()
    }
}
