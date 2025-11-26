use alloy::{
    primitives::{Address, Bytes, U256, keccak256},
    sol_types::SolValue,
};
use ark_bn254::Fr;
use ark_ff::PrimeField;

pub struct ExtraCommitment {
    pub receiver: Address,
    pub prover_fee: U256,
    pub broadcaster_fee: U256,
    pub receiver_hook: Bytes,
}

impl ExtraCommitment {
    pub fn new(
        receiver: Address,
        prover_fee: U256,
        broadcaster_fee: U256,
        receiver_hook: Bytes,
    ) -> Self {
        Self {
            receiver,
            prover_fee,
            broadcaster_fee,
            receiver_hook,
        }
    }

    pub fn hash(&self) -> Fr {
        Fr::from_be_bytes_mod_order(
            &keccak256(
                (
                    self.broadcaster_fee,
                    self.prover_fee,
                    self.receiver,
                    &self.receiver_hook,
                )
                    .abi_encode_packed()
                    .as_slice(),
            )
            .0,
        )
    }
}
