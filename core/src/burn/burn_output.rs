use alloy::primitives::{Address, Bytes, U256};
use ark_bn254::Fr;

use crate::contracts::network::Network;

pub struct BurnOutput {
    // So we don't need to get it from user again on burn
    pub network: Network,

    // Burn_address 2nd param
    pub burn_key: Fr, // Save as string because it's a pretty large number

    // Burn_address 3rd param
    pub burn_amount: U256, // Save as string because it's a pretty large number
    pub reveal_amount: U256, // Save as string because it's a pretty large number

    // Extra commitment content
    pub receiver: Address,
    pub prover_fee: U256,
    pub broadcaster_fee: U256,
    pub receiver_hook: Bytes,
}

impl BurnOutput {
    pub fn new(
        network: Network,
        burn_key: Fr,
        burn_amount: U256,
        reveal_amount: U256,
        receiver: Address,
        prover_fee: U256,
        broadcaster_fee: U256,
        receiver_hook: Bytes,
    ) -> Self {
        Self {
            network,
            burn_key,
            burn_amount,
            reveal_amount,
            receiver,
            prover_fee,
            broadcaster_fee,
            receiver_hook,
        }
    }
}
