pub mod burn_address;
pub mod error;
pub mod extra_commitment;
pub mod poseidon4;

use std::str::FromStr;

use alloy::primitives::Address;
use ark_bn254::Fr;

use crate::{
    burn::{
        burn_address::{burn_address, burn_key::new_burn_key},
        error::BurnError,
        extra_commitment::new_extra_commitment,
    },
    contracts::network::Network,
};

pub async fn burn(
    network: Network,
    private_key: String,
    amount: u64,
    reveal: u64,
    broadcaster_fee: u64,
    broadcaster: String,
    sell_on_uniswap: u64,
    receiver_address: String,
) -> Result<(), BurnError> {
    let burn_key = new_burn_key();
    println!("Your burn_key: `{}`", burn_key);

    let receiver_address = Address::from_str(&receiver_address)
        .map_err(|e| BurnError::validation("receiver_address", receiver_address, e.to_string()))?;

    let extra_commitment =
        new_extra_commitment(receiver_address, prover_fee, broadcaster_fee, receiver_hook);

    let burn_address = burn_address(burn_key, Fr::from(reveal), extra_commitment)?;

    println!("your burn address: `{}`", burn_address);
    Ok(())
}
