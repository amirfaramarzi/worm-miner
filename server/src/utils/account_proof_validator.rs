use crate::error::ServerError;
use alloy::{
    primitives::{B256, Bytes, FixedBytes, U256, keccak256},
    rpc::types::EIP1186AccountProofResponse,
};
use alloy_rlp::RlpEncodable;
use anyhow::anyhow;

#[derive(Debug, RlpEncodable)]
struct MptLeaf {
    nonce: u64,
    balance: U256,
    storage_hash: FixedBytes<32>,
    code_hash: FixedBytes<32>,
}

pub fn validate_account_proof(
    proof: EIP1186AccountProofResponse,
    state_root: B256,
) -> Result<(), ServerError> {
    if keccak256(&proof.account_proof[0]) != state_root {
        return proof_err("State-root inconsistency!");
    }

    for i in 1..proof.account_proof.len() {
        let hash = keccak256(&proof.account_proof[i]);
        if !proof.account_proof[i - 1].windows(32).any(|w| w == hash) {
            return proof_err("Layer inconsistency!");
        }
    }

    let leaf = MptLeaf {
        nonce: proof.nonce,
        balance: proof.balance,
        storage_hash: proof.storage_hash,
        code_hash: proof.code_hash,
    };
    let leaf_rlp = alloy::rlp::encode(&leaf);
    let last_layer = proof
        .account_proof
        .last()
        .ok_or(anyhow!("Account proof empty"))?;

    let addr_hash = keccak256(proof.address);
    let key_val: Vec<Bytes> = alloy::rlp::decode_exact(&last_layer)
        .map_err(|_| proof_err("Last layer decode").err().unwrap())?;

    if key_val.len() != 2 {
        return proof_err("Weird leaf!");
    }

    if key_val[1] != leaf_rlp {
        return proof_err("Account inconsistency!");
    }

    let key_hex = if key_val[0][0] & 0xf0 == 0x30 {
        String::from(&format!("{:x}", key_val[0])[3..])
    } else if key_val[0][0] == 0x20 {
        String::from(&format!("{:x}", key_val[0])[4..])
    } else {
        return proof_err("Weird address hash prefix!");
    };

    if !format!("{:x}", addr_hash).ends_with(&key_hex) {
        return proof_err("Address inconsistency!");
    }

    Ok(())
}

fn proof_err(msg: &'static str) -> Result<(), ServerError> {
    Err(ServerError::validation("account_proof", "", msg))
}
