use std::path::PathBuf;

use alloy::primitives::U256;
use serde::{Deserialize, Serialize};

use crate::utils::worm_home;

pub fn generate_proof(witness_file: PathBuf) -> Result<RapidsnarkOutput, anyhow::Error> {
    let zkey = worm_home::get_proof_of_burn_zkey()?;
    let params = std::fs::read(zkey)?;

    let witness = std::fs::read(witness_file)?;

    let proof = worm_witness_gens::rapidsnark(&params, &witness)?;
    let proof_proof: RapidsnarkProof = serde_json::from_str(&proof.proof)?;
    let proof_public: Vec<alloy::primitives::U256> = serde_json::from_str(&proof.public)?;
    let output = RapidsnarkOutput {
        proof: proof_proof,
        public: proof_public,
    };

    Ok(output)
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RapidsnarkProof {
    pub pi_a: [U256; 3],
    pub pi_b: [[U256; 2]; 3],
    pub pi_c: [U256; 3],
    pub protocol: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RapidsnarkOutput {
    pub proof: RapidsnarkProof,
    pub public: Vec<U256>,
}

impl RapidsnarkOutput {
    pub fn to_json(&self) -> String {
        serde_json::to_string(self).unwrap()
    }
}
