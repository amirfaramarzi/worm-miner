use alloy::primitives::{Address, U256};
use serde::{Deserialize, Serialize};

use crate::mint::witness_input_file::WitnessInputFile;

pub fn generate_witness(
    witness_input_file: WitnessInputFile,
    burn_address: Address,
) -> Result<RapidsnarkOutput, anyhow::Error> {
    // random file name in temp
    let input_json_path = std::env::temp_dir().join(format!("{}.json", burn_address.to_string()));
    witness_input_file.write_to_file(input_json_path)?;

    todo!()
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
