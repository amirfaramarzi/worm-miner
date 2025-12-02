pub mod broadcaster;
pub mod burn_address;
pub mod burn_output;
pub mod error;
pub mod extra_commitment;

use alloy::{
    primitives::*,
    providers::{Provider, ProviderBuilder},
    rpc::types::TransactionRequest,
    signers::local::PrivateKeySigner,
};

use crate::{
    burn::{
        burn_address::{burn_address, burn_key::find_burn_key},
        burn_output::BurnOutput,
        error::BurnError,
        extra_commitment::ExtraCommitment,
    },
    contracts::network::Network,
    utils::{ToU256, TryToFr},
};

pub async fn burn(
    network: Network,
    private_key: PrivateKeySigner,
    amount: U256,
    reveal: U256,
    broadcaster_fee: U256,
    sell_on_uniswap: U256,
    receiver_address: Address,
    prover_fee: U256,
) -> Result<(BurnOutput, Address), BurnError> {
    let receiver_hook = if sell_on_uniswap == 0 {
        Bytes::new()
    } else {
        Bytes::new() // TODO make calldata with uniswap interface
    };

    let extra_commitment = ExtraCommitment::new(
        receiver_address,
        prover_fee,
        broadcaster_fee,
        receiver_hook.clone(),
    );

    let burn_key = find_burn_key(extra_commitment.hash().unwrap().to_u256(), reveal);
    println!("Your burn_key: `{}`", burn_key);

    let burn_address = burn_address(
        burn_key,
        reveal
            .try_to_fr()
            .map_err(|x| Into::<anyhow::Error>::into(x))?,
        extra_commitment,
    )?;

    println!("your burn address: `{}`", burn_address);

    let provider = ProviderBuilder::new()
        .wallet(private_key)
        .connect(network.url())
        .await?;

    let tx = TransactionRequest::default().to(burn_address).value(amount);

    let pending = provider.send_transaction(tx).await?;
    println!("pending:\n{:?}", pending);

    let receipt = pending.get_receipt().await?;
    println!("receipt:\n{:?}", receipt);

    Ok((
        BurnOutput::new(
            network,
            burn_key,
            reveal,
            receiver_address,
            prover_fee,
            broadcaster_fee,
            receiver_hook,
        ),
        burn_address,
    ))
}
