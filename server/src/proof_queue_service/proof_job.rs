use crate::data::config::Config;
use crate::handlers::ProofPostRequest;
use alloy::consensus::Header;
use alloy::primitives::U256;
use anyhow::Ok;
use common::burn::burn_address::burn_address;
use common::burn::extra_commitment;
use common::mint::proof_generator::generate_proof;
use common::mint::witness_generator::generate_witness;
use common::mint::witness_input_file::WitnessInputFile;
use common::mint::{nullifier, proof_generator::RapidsnarkOutput};
use common::utils::{ToU256, TryToFr, fr_to_u256, u256_to_fr};

pub type ProofResult = RapidsnarkOutput;

pub struct ProofJob {
    pub proof_input: ProofPostRequest,
    pub header: Header,
    pub nullifier: U256,
}

impl ProofJob {
    pub fn new(proof_input: ProofPostRequest, header: Header) -> Self {
        let nullifier_ =
            nullifier::compute_nullifier(u256_to_fr(proof_input.burn_key).unwrap()).unwrap();
        Self {
            proof_input,
            header,
            nullifier: fr_to_u256(&nullifier_),
        }
    }
}

impl ProofJob {
    pub async fn run(&self, config: Config) -> Result<ProofResult, anyhow::Error> {
        // TODO
        let inp = &self.proof_input;
        let burn_key = inp.burn_key.try_to_fr()?;
        let burn_extra_commitment = extra_commitment::ExtraCommitment::new(
            inp.receiver_address,
            inp.prover_fee,
            inp.broadcaster_fee,
            inp.receiver_hook.clone(),
        );

        let witness_input_file = WitnessInputFile::new(
            inp.account_proof.clone(),
            self.header.clone(),
            burn_key,
            inp.spend,
            burn_extra_commitment.hash()?,
            config.owner_address,
        )?;
        let burn_address = burn_address(burn_key, inp.spend.try_to_fr()?, burn_extra_commitment)?;

        let witness_file = generate_witness(witness_input_file, burn_address).unwrap();

        let proof = generate_proof(witness_file)?;

        Ok(proof)
    }
}
