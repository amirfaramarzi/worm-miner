use crate::handlers::ProofPostRequest;
use alloy::primitives::U256;
use common::mint::{nullifier, proof_generator::RapidsnarkOutput};
use common::utils::{fr_to_u256, u256_to_fr};

pub type ProofResult = RapidsnarkOutput;

pub struct ProofJob {
    pub proof_input: ProofPostRequest,
    pub nullifier: U256,
}

impl ProofJob {
    pub fn new(proof_input: ProofPostRequest) -> Self {
        let nullifier_ =
            nullifier::compute_nullifier(u256_to_fr(proof_input.burn_key).unwrap()).unwrap();
        Self {
            proof_input,
            nullifier: fr_to_u256(&nullifier_),
        }
    }
}

impl ProofJob {
    pub async fn run(&self) -> ProofResult {
        // TODO
        todo!()
    }
}
