use alloy::primitives::U256;
use rand::RngCore;

pub fn new_burn_key() -> U256 {
    // let mut r = rand::random();
    let mut rng = rand::rng();

    // Method 1: Generate 4 random u64s and construct U256
    let limbs = [
        rng.next_u64(),
        rng.next_u64(),
        rng.next_u64(),
        rng.next_u64(),
    ];
    U256::from_limbs(limbs)
}
