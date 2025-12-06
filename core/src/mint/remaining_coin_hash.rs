use alloy::primitives::U256;
use anyhow::anyhow;
use ark_bn254::Fr;

use crate::utils::{TryToFr, coin_prefix, poseidon3};

pub fn compute_remaining_coin(
    burn_key: Fr,
    amount: U256,
    spend: U256,
) -> Result<Fr, anyhow::Error> {
    if spend > amount {
        return Err(anyhow!("Spend amount must be <= amount"));
    }
    let remaining_coin = (amount - spend).try_to_fr()?;
    Ok(poseidon3(coin_prefix(), burn_key, remaining_coin)?)
}
