use std::fmt::Display;

use alloy::primitives::U256;
use ark_bn254::Fr;
use ark_ff::PrimeField;

pub fn u256_to_fr(value: U256) -> Result<Fr, ToFrOverFlowError> {
    let limbs: [u64; 4] = value.into_limbs();
    let bigint = <Fr as PrimeField>::BigInt::new(limbs);
    Fr::from_bigint(bigint).ok_or(ToFrOverFlowError)
}

pub fn fr_to_u256(value: &Fr) -> U256 {
    let bigint = value.into_bigint();
    let limbs = bigint.0;
    U256::from_limbs([limbs[0], limbs[1], limbs[2], limbs[3]])
}

pub trait TryToFr {
    fn try_to_fr(self) -> Result<Fr, ToFrOverFlowError>;
}

pub trait ToU256 {
    fn to_fr(&self) -> U256;
}

impl TryToFr for U256 {
    fn try_to_fr(self) -> Result<Fr, ToFrOverFlowError> {
        u256_to_fr(self)
    }
}

impl ToU256 for Fr {
    fn to_fr(&self) -> U256 {
        fr_to_u256(self)
    }
}

#[derive(Clone, Copy, Debug)]
pub struct ToFrOverFlowError;

impl Display for ToFrOverFlowError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("error while converting to Fr because number is big")
    }
}

use anyhow;
impl From<ToFrOverFlowError> for anyhow::Error {
    fn from(e: ToFrOverFlowError) -> Self {
        anyhow::anyhow!(e.to_string())
    }
}
