use axum::response::IntoResponse;

/// GET `/proof` returns minimum proving fee of the relayer.
pub async fn proof_get() -> ProofGetResponse {
    todo!();
}

pub struct ProofGetResponse {}

impl IntoResponse for ProofGetResponse {
    fn into_response(self) -> axum::response::Response {
        todo!()
    }
}
