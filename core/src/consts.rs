use std::sync::LazyLock;

use alloy::primitives::{U256, utils::parse_ether};

pub static BURN_AMOUNT_LIMIT: LazyLock<U256> = LazyLock::new(|| parse_ether("10").unwrap());
