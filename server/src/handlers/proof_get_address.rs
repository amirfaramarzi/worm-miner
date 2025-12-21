use axum::{extract::Path, response::IntoResponse};

///  GET `/proof/{burn-address}` gets cached proof for the given burn-address.
pub async fn proof_get_address(Path(burn_address): Path<String>) -> ProofGetAddressResponse {
    println!("burn addresss: `{}`", burn_address);
    todo!();
}

pub struct ProofGetAddressResponse {}

impl IntoResponse for ProofGetAddressResponse {
    fn into_response(self) -> axum::response::Response {
        todo!()
    }
}
