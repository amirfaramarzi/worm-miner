use std::path::PathBuf;

use crate::{mint::witness_input_file::WitnessInputFile, utils::worm_home};
use alloy::primitives::Address;

/// returns witness_file_path
pub fn generate_witness(
    witness_input_file: WitnessInputFile,
    burn_address: Address,
) -> Result<PathBuf, anyhow::Error> {
    // random file name in temp
    let input_json_path = std::env::temp_dir().join(format!("{}.json", burn_address.to_string()));
    witness_input_file.write_to_file(&input_json_path)?;

    let witness_file_path = std::env::current_dir()?.join("./witness.wtns");

    let proof_of_burn_dat = worm_home::get_proof_of_burn_dat()?;
    worm_witness_gens::generate_proof_of_burn_witness_file(
        proof_of_burn_dat,
        input_json_path,
        &witness_file_path,
    )?;
    Ok(witness_file_path)
}
