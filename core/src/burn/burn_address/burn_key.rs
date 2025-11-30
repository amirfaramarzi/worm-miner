use ark_bn254::Fr;
use ark_ff::BigInt;
use rand::RngCore;

pub fn new_burn_key() -> Fr {
    let mut rng = rand::rng();

    let limbs = [
        rng.next_u64(),
        rng.next_u64(),
        rng.next_u64(),
        rng.next_u64(),
    ];
    Fr::new(BigInt::new(limbs))
}
