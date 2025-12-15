pub mod burn_key;

use alloy::primitives::Address;
use ark_bn254::Fr;
use ark_ff::{BigInteger, PrimeField};

use crate::{
    burn::extra_commitment::ExtraCommitment,
    utils::{burn_address_prefix, poseidon},
};

pub fn burn_address(
    burn_key: Fr,
    reveal_amount: Fr,
    extra_commitment: ExtraCommitment,
) -> Result<Address, anyhow::Error> {
    let hashed = poseidon::poseidon4(
        burn_address_prefix(),
        burn_key,
        reveal_amount,
        extra_commitment.hash()?,
    )?;
    let bytes = hashed.into_bigint().to_bytes_be();
    let address = Address::from_slice(&bytes[0..20]);
    Ok(address)
}
