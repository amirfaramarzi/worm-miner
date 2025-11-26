use alloy::primitives::U256;
use ark_bn254::Fr;
use ark_ff::PrimeField;

pub fn u256_to_fr_mod(value: U256) -> Fr {
    let le_bytes = value.to_le_bytes::<32>();
    Fr::from_le_bytes_mod_order(&le_bytes)
}

pub fn fr_to_u256(value: &Fr) -> U256 {
    let bigint = value.into_bigint();
    let limbs = bigint.0;
    U256::from_limbs([limbs[0], limbs[1], limbs[2], limbs[3]])
}

pub trait ToFr {
    fn to_fr(self) -> Fr;
}

pub trait ToU256 {
    fn to_fr(&self) -> U256;
}

impl ToFr for U256 {
    fn to_fr(self) -> Fr {
        u256_to_fr_mod(self)
    }
}

impl ToU256 for Fr {
    fn to_fr(&self) -> U256 {
        fr_to_u256(self)
    }
}
