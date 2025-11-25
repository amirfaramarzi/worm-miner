use alloy::primitives::U256;
use ark_bn254::Fr;

/// This is the base constant value for the Poseidon prefix,
/// derived from keccak256("EIP-7503") mod P
const POSEIDON_PREFIX_VALUE_STR: &str =
    "5265656504298861414514317065875120428884240036965045859626767452974705356670";

pub fn prefix() -> U256 {
    U256::from_str_radix(POSEIDON_PREFIX_VALUE_STR, 10).unwrap()
}
