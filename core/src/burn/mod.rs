pub mod broadcaster;
pub mod burn_address;
pub mod error;
pub mod extra_commitment;
pub mod poseidon4;

use alloy::{primitives::*, signers::local::PrivateKeySigner};

use crate::{
    burn::{
        broadcaster::Broadcaster,
        burn_address::{burn_address, burn_key::new_burn_key},
        error::BurnError,
        extra_commitment::ExtraCommitment,
    },
    contracts::network::Network,
    utils::ToFr,
};

pub async fn burn(
    network: Network,
    private_key: PrivateKeySigner,
    amount: U256,
    reveal: U256,
    broadcaster_fee: U256,
    broadcaster: Broadcaster,
    sell_on_uniswap: U256,
    receiver_address: Address,
    prover_fee: U256,
) -> Result<(), BurnError> {
    let receiver_hook = if sell_on_uniswap == 0 {
        Bytes::new()
    } else {
        Bytes::new() // TODO make calldata with uniswap interface
    };

    let extra_commitment =
        ExtraCommitment::new(receiver_address, prover_fee, broadcaster_fee, receiver_hook);

    let burn_key = new_burn_key();
    println!("Your burn_key: `{}`", burn_key);

    let burn_address = burn_address(burn_key, reveal.to_fr(), extra_commitment)?;

    println!("your burn address: `{}`", burn_address);
    Ok(())
}
