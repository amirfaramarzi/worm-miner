pub mod error;
pub mod proof_generator;
pub mod witness_generator;
pub mod witness_input_file;

use alloy::{primitives::Address, rlp::Encodable, signers::local::PrivateKeySigner};

use crate::{
    burn::{
        burn_address::burn_address, burn_output::BurnOutput, extra_commitment::ExtraCommitment,
    },
    mint::{
        error::MintError, proof_generator::generate_proof, witness_generator::generate_witness,
        witness_input_file::WitnessInputFile,
    },
    utils::TryToFr,
};
use alloy::{
    eips::BlockId,
    primitives::keccak256,
    providers::{Provider, ProviderBuilder},
};
use anyhow::anyhow;

/// [singer] who calls mint() of BETH and pays gas fee
/// [prover_address] who gets prover_fee
pub async fn mint(
    burn_output: BurnOutput,
    prover_address: Address,
    signer: PrivateKeySigner,
) -> Result<(), MintError> {
    let burn_extra_commitment = ExtraCommitment::new(
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
        burn_extra_commitment.clone(),
    )?;

    let provider = ProviderBuilder::new()
        .connect(burn_output.network.url())
        .await?;

    // making sure that proof and block_header are pointing to same block
    let (block_header, proof) = loop {
        let block = provider
            .get_block(BlockId::latest())
            .await?
            .ok_or(anyhow!("block not found"))?;
        let proof = provider.get_proof(burn_address, vec![]).await?;
        if block.header.state_root == keccak256(&proof.account_proof[0]) {
            let mut block_header = vec![];
            block.header.inner.encode(&mut block_header);
            break (block_header, proof);
        }
    };

    let witness_input_file = WitnessInputFile::new(
        proof,
        block_header,
        burn_output.burn_key,
        burn_output.reveal_amount,
        burn_extra_commitment.hash().unwrap(),
        prover_address,
    )?;

    println!("Generating witness...");
    let witness_file = generate_witness(witness_input_file, burn_address)?;

    println!("Generating proof...");
    let proof = generate_proof(witness_file)?;

    println!("Proof:\n{}", proof.to_json());

    todo!()
}
