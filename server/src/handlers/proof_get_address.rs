use crate::data::AppState;
use axum::{
    extract::{Path, State},
    response::IntoResponse,
};
use std::sync::{Arc, RwLock};

///  GET `/proof/{burn-address}` gets cached proof for the given burn-address.
pub async fn proof_get_address(
    State(state): State<Arc<RwLock<AppState>>>,
    Path(burn_address): Path<String>,
) -> ProofGetAddressResponse {
    println!("burn addresss: `{}`", burn_address);
    todo!();
}

pub struct ProofGetAddressResponse {}

impl IntoResponse for ProofGetAddressResponse {
    fn into_response(self) -> axum::response::Response {
        todo!()
    }
}
