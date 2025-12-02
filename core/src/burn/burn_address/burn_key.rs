use alloy::primitives::{U256, keccak256};
use ark_bn254::Fr;
use ark_ff::BigInt;
use rand::RngCore;

use crate::utils::{ToU256, TryToFr};

pub fn find_burn_key(burn_extra_commit: U256, reveal: U256) -> Fr {
    let pow_min_zero_bytes: usize = 2;
    let mut curr: U256 = random_fr().to_u256();

    loop {
        let mut inp: [u8; 104] = [0; 104];
        inp[..32].copy_from_slice(&curr.to_be_bytes::<32>());
        inp[32..64].copy_from_slice(&reveal.to_be_bytes::<32>());
        inp[64..96].copy_from_slice(&burn_extra_commit.to_be_bytes::<32>());
        inp[96..].copy_from_slice(b"EIP-7503");
        let hash: U256 = keccak256(inp).into();
        if hash.leading_zeros() >= pow_min_zero_bytes * 8 {
            return curr.try_to_fr().unwrap();
        }
        curr += U256::ONE;
    }
}

pub fn random_fr() -> Fr {
    let mut rng = rand::rng();

    let limbs = [
        rng.next_u64(),
        rng.next_u64(),
        rng.next_u64(),
        rng.next_u64(),
    ];
    Fr::new(BigInt::new(limbs))
}
