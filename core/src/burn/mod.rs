pub mod burn_address;
pub mod error;
pub mod poseidon4;

use alloy::primitives::U256;

use crate::{
    burn::{
        burn_address::{burn_address, burn_key::new_burn_key},
        error::BurnError,
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
) -> Result<(), BurnError> {
    let burn_key = new_burn_key();
    println!("Your burn_key: `{}`", burn_key);
    let extra_commitment = U256::from(1); // TODO calculate extra commitment
    let burn_address = burn_address(burn_key, U256::from(reveal), extra_commitment)?;
    println!("your burn address: `{}`", burn_address);
    Ok(())
}
