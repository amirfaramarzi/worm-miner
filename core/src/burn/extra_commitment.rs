use alloy::{
    primitives::{Address, Bytes, U256, keccak256},
    sol_types::SolValue,
};
use ark_bn254::Fr;
use ark_ff::PrimeField;

pub fn new_extra_commitment(
    receiver: Address,
    prover_fee: U256,
    broadcaster_fee: U256,
    receiver_hook: Bytes,
) -> Fr {
    Fr::from_be_bytes_mod_order(
        &keccak256(
            (broadcaster_fee, prover_fee, receiver, receiver_hook)
                .abi_encode_packed()
                .as_slice(),
        )
        .0,
    )
}
