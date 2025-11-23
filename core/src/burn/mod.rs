pub mod burn_address;
pub mod error;
pub mod poseidon4;

use crate::{burn::error::BurnError, contracts::network::Network};

pub async fn burn(
    network: Network,
    private_key: String,
    amount: u64,
    reveal: u64,
    broadcaster_fee: u64,
    broadcaster: String,
    sell_on_uniswap: u64,
) -> Result<(), BurnError> {
    println!("Burning...");
    Ok(())
}
