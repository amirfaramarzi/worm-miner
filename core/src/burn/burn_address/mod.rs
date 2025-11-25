pub mod burn_key;
pub mod prefix;

use alloy::primitives::{Address, U256};

use crate::burn::{burn_address::prefix::prefix, poseidon4};

pub fn burn_address(
    burn_key: U256,
    reveal_amount: U256,
    extra_commitment: U256,
) -> Result<Address, anyhow::Error> {
    let hashed = poseidon4::poseidon4(prefix(), burn_key, reveal_amount, extra_commitment)?;
    let bytes: [u8; 32] = hashed.to_be_bytes();
    let address = Address::from_slice(&bytes[0..20]);
    Ok(address)
}
