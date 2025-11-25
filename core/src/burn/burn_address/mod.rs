pub mod burn_key;
pub mod prefix;

use alloy::primitives::Address;

use crate::burn::burn_address::{burn_key::new_burn_key, prefix::prefix};

pub fn burn_address() -> Result<Address, anyhow::Error> {
    let p1 = prefix();
    let p2 = new_burn_key();
    todo!()
}
