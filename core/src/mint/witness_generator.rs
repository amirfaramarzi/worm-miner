use crate::{mint::witness_input_file::WitnessInputFile, utils::worm_home};
use alloy::primitives::{Address, U256};
use serde::{Deserialize, Serialize};

pub fn generate_witness(
    witness_input_file: WitnessInputFile,
    burn_address: Address,
) -> Result<RapidsnarkOutput, anyhow::Error> {
    // random file name in temp
    let input_json_path = std::env::temp_dir().join(format!("{}.json", burn_address.to_string()));
    witness_input_file.write_to_file(&input_json_path)?;

    let witness_file_path = std::env::current_dir()?.join("./witness.wtns");

    let proof_of_burn_dat = worm_home::get_proof_of_burn_dat()?;
    worm_witness_gens::generate_proof_of_burn_witness_file(
        proof_of_burn_dat,
        input_json_path,
        witness_file_path,
    )?;
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
