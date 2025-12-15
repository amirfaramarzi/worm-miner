use crate::utils::{nullifier_prefix, poseidon2};
use ark_bn254::Fr;

pub fn compute_nullifier(burn_key: Fr) -> Result<Fr, anyhow::Error> {
    poseidon2(nullifier_prefix(), burn_key)
}
