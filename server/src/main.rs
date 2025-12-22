mod data;
mod error;
mod handlers;

use axum::{
    Router,
    routing::{get, post},
};

use crate::{
    data::{AppState, config::Config},
    handlers::*,
};

#[tokio::main]
async fn main() {
    let config = Config::load().expect("error file loading config file");
    let port = config.port;
    config.print();
    let state = AppState::new(config);

    let router = Router::new()
        .route("/proof", get(proof_get))
        .route("/proof", post(proof_post))
        .route("/proof/{nullifier}", get(proof_get_by_nullifier))
        .route("/relay", get(relay_get))
        .route("/relay", post(relay_post))
        .with_state(state);

    let address = format!("0.0.0.0:{}", port);
    let listener = tokio::net::TcpListener::bind(&address).await.unwrap();
    println!("Listening on `{}`", address);
    axum::serve(listener, router).await.unwrap();
}
