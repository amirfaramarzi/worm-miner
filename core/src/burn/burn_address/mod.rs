pub mod burn_key;
pub mod prefix;

use alloy::primitives::Address;
use ark_bn254::Fr;
use ark_ff::{BigInteger, PrimeField};

use crate::burn::{burn_address::prefix::prefix, extra_commitment::ExtraCommitment, poseidon4};

pub fn burn_address(
    burn_key: Fr,
    reveal_amount: Fr,
    extra_commitment: ExtraCommitment,
) -> Result<Address, anyhow::Error> {
    let hashed = poseidon4::poseidon4(prefix(), burn_key, reveal_amount, extra_commitment.hash()?)?;
    let bytes = hashed.into_bigint().to_bytes_be();
    let address = Address::from_slice(&bytes[0..20]);
    Ok(address)
}
