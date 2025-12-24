use crate::data::config::Config;
use crate::handlers::ProofPostRequest;
use alloy::consensus::Header;
use alloy::primitives::U256;
use anyhow::Ok;
use anyhow::anyhow;
use common::burn::burn_address::burn_address;
use common::burn::extra_commitment;
use common::mint::nullifier;
use common::mint::proof_generator::RapidsnarkOutput;
use common::mint::proof_generator::generate_proof;
use common::mint::witness_generator::generate_witness_with_files;
use common::mint::witness_input_file::WitnessInputFile;
use common::utils::{TryToFr, fr_to_u256, u256_to_fr};
use std::env::{self};
use std::fs;
use std::path::PathBuf;
use std::process::exit;

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
    pub async fn run(&self, config: Config) -> Result<RapidsnarkOutput, anyhow::Error> {
        let inp = &self.proof_input;
        let burn_key = inp.burn_key.try_to_fr()?;
        let burn_extra_commitment = extra_commitment::ExtraCommitment::new(
            inp.receiver_address,
            inp.prover_fee,
            inp.broadcaster_fee,
            inp.receiver_hook.clone(),
        );
        let burn_extra_commitment_hash = burn_extra_commitment.hash()?;
        let burn_address = burn_address(burn_key, inp.spend.try_to_fr()?, burn_extra_commitment)?;

        let witness_input_file = WitnessInputFile::new(
            inp.account_proof.clone(),
            self.header.clone(),
            burn_key,
            inp.spend,
            burn_extra_commitment_hash,
            config.owner_address,
        )?;

        let input_wit_file =
            std::env::temp_dir().join(format!("witness_input_{}.json", burn_address));

        let output_wit_path = std::env::temp_dir().join(format!("./witness_{}.wtns", burn_address));

        let proof_file_path = std::env::temp_dir().join(format!("./proof_{}.json", burn_address));

        tokio::fs::write(&input_wit_file, witness_input_file.to_json()).await?;

        let mut child = tokio::process::Command::new(std::env::current_exe()?)
            .arg("rapidsnark")
            .arg(input_wit_file)
            .arg(output_wit_path)
            .arg(&proof_file_path)
            .spawn()
            .expect("Failed to spawn child process");

        let status = child.wait().await?;

        if let Some(code) = status.code() {
            // non-zero code
            let logs = child.stderr;
            return Err(anyhow!(
                "error code `{}` on child process while generating proof, logs:\n {:?}",
                code,
                logs
            ));
        }

        let proof = serde_json::from_str(&tokio::fs::read_to_string(proof_file_path).await?)?;

        Ok(proof)
    }
}

/// call this on main first line
/// it will do nothing on server mode
/// and creates proof and exit on child mode
pub fn run_rapidsnark_task_if_child_process() {
    let args: Vec<String> = env::args().collect();

    if args.len() > 1 && args[1] == "rapidsnark" {
        let wit_input = PathBuf::from(args.get(2).expect("pass witness_input path as 2nd arg"));
        let wit_output = PathBuf::from(args.get(3).expect("pass witness_input path as 3rd arg"));
        let proof = PathBuf::from(args.get(4).expect("pass witness_input path as 4th arg"));

        let result = child_process_handler(wit_input, wit_output, proof);

        if let Err(e) = result {
            eprintln!("{}", e);
            exit(1); // non-zero code
        }

        // this prevents server start
        // and zero code means operation has finished successfully
        exit(0);
    }
}

pub fn child_process_handler(
    witness_input: PathBuf,
    witness_output: PathBuf,
    proof_file: PathBuf,
) -> Result<(), anyhow::Error> {
    let witness_file = generate_witness_with_files(witness_input, witness_output)?;
    let proof = generate_proof(witness_file)?;
    fs::write(proof_file, serde_json::to_string_pretty(&proof)?)?;
    Ok(())
}
