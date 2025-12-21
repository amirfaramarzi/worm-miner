mod data;
mod handlers;

use axum::{
    Router,
    routing::{get, post},
};

use crate::{data::AppState, handlers::*};

#[tokio::main]
async fn main() {
    let state = AppState::new();

    let router = Router::new()
        .route("/proof", get(proof_get))
        .route("/proof", post(proof_post))
        .route("/proof/{burn_address}", get(proof_get_address))
        .route("/relay", get(relay_get))
        .route("/relay", post(relay_post))
        .with_state(state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, router).await.unwrap();
}
