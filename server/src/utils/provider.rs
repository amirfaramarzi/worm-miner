use crate::error::ServerError;
use alloy::{network::Ethereum, providers::RootProvider};
use anyhow::anyhow;
use common::contracts::network::Network;

pub fn get_provider(network: Network) -> Result<RootProvider<Ethereum>, ServerError> {
    let url = network
        .url()
        .try_into()
        .map_err(|e| ServerError::Unexpected(anyhow!("{e}").into_boxed_dyn_error()))?;
    let provider = RootProvider::new_http(url);
    return Ok(provider);
}
