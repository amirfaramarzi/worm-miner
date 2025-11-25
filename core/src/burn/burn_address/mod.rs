pub mod prefix;

use alloy::primitives::Address;

use crate::burn::burn_address::prefix::prefix;

pub fn burn_address() -> Result<Address, anyhow::Error> {
    let p1 = prefix();
    todo!()
}
