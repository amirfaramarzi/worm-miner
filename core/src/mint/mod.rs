pub mod error;
pub mod witness_input_file;

use crate::{
    burn::{
        broadcaster::Broadcaster, burn_address::burn_address, burn_output::BurnOutput,
        extra_commitment::ExtraCommitment,
    },
    mint::error::MintError,
    utils::TryToFr,
};
use alloy::providers::{Provider, ProviderBuilder};
use anyhow::anyhow;

pub async fn mint(burn_output: BurnOutput, broadcaster: Broadcaster) -> Result<(), MintError> {
    let extra_commitment = ExtraCommitment::new(
        burn_output.receiver,
        burn_output.prover_fee,
        burn_output.broadcaster_fee,
        burn_output.receiver_hook.clone(),
    );

    let burn_address = burn_address(
        burn_output.burn_key,
        burn_output
            .reveal_amount
            .try_to_fr()
            .map_err(|e| anyhow!("{e}"))?,
        extra_commitment,
    )?;

    let provider = ProviderBuilder::new()
        .connect(burn_output.network.url())
        .await?;

    let proof = provider.get_proof(burn_address, vec![]).await?;
    dbg!(proof);
    todo!()
}
