use std::str::FromStr;

use ark_bn254::Fr;
use ark_ff::BigInt;

/// This is the base constant value for the Poseidon prefix,
/// derived from keccak256("EIP-7503") mod P
const POSEIDON_PREFIX_VALUE_STR: &str =
    "5265656504298861414514317065875120428884240036965045859626767452974705356670";

pub fn burn_address_prefix() -> Fr {
    Fr::new(BigInt::<4>::from_str(POSEIDON_PREFIX_VALUE_STR).unwrap())
}

pub fn nullifier_prefix() -> Fr {
    burn_address_prefix() + Fr::from(1)
}

pub fn coin_prefix() -> Fr {
    burn_address_prefix() + Fr::from(2)
}
