use std::{fs, path::PathBuf};

use crate::consts::BURN_AMOUNT_LIMIT;
use alloy::{
    primitives::{Address, U256, keccak256},
    rlp::Encodable,
    rlp::RlpDecodable,
    rpc::types::{Block, EIP1186AccountProofResponse},
};
use alloy_rlp::Decodable;
use anyhow::anyhow;
use ark_bn254::Fr;

// number of layers circuit expect us
const NUMBER_OF_LAYERS: usize = 16;

// number of bytes in each layer
const LAYER_LEN: usize = 4 * 136;

const MAX_HEADER_LEN: usize = 16 * 136;

/// Warning: Do NOT rename this fields because these should be same as circom signals names
#[allow(non_snake_case)]
#[derive(serde::Deserialize, serde::Serialize)]
pub struct WitnessInputFile {
    actualBalance: String,
    intendedBalance: String,
    numLayers: usize,
    layerLens: Vec<usize>,
    layers: Vec<Vec<u8>>,
    blockHeader: Vec<u8>,
    blockHeaderLen: usize,
    numLeafAddressNibbles: String,
    burnKey: String,
    revealAmount: String,
    burnExtraCommitment: String,
    byteSecurityRelax: i32,
    _proofExtraCommitment: String,
}

impl WitnessInputFile {
    pub fn new(
        proof: EIP1186AccountProofResponse,
        block: Block,
        burn_key: Fr,
        spend: U256,
        burn_extra_commitment: Fr,
        prover: Address,
    ) -> Result<Self, WitnessInputFileError> {
        if !block.header.state_root == keccak256(&proof.account_proof[0]) {
            return Err(WitnessInputFileError::NotOnSameBlock);
        }

        let mut header_bytes = vec![];
        block.header.inner.encode(&mut header_bytes);

        let leaf = proof
            .account_proof
            .last()
            .ok_or(anyhow!("Leaf doesn't exist!"))?;
        let rlp_leaf = RlpLeaf::decode(&mut leaf.as_ref()).map_err(|e| anyhow!("{e}"))?;
        let num_addr_hash_nibbles = if (rlp_leaf.key[0] & 0xf0) == 0x20 {
            2 * rlp_leaf.key.len() - 2
        } else if (rlp_leaf.key[0] & 0xf0) == 0x30 {
            2 * rlp_leaf.key.len() - 1
        } else {
            return Err(anyhow!("Unexpected leaf-key prefix!"))?;
        };

        let mut layers = vec![];
        for layer in proof.account_proof.iter() {
            let mut extended_layer = layer.to_vec();
            extended_layer.resize(LAYER_LEN, 0);
            layers.push(extended_layer);
        }
        while layers.len() < NUMBER_OF_LAYERS {
            layers.push(vec![0; LAYER_LEN]);
        }
        let mut layer_bits_lens = proof
            .account_proof
            .iter()
            .map(|l| l.len())
            .collect::<Vec<_>>();
        layer_bits_lens.resize(NUMBER_OF_LAYERS, 32);
        let mut extended_header = header_bytes.to_vec();
        extended_header.resize(MAX_HEADER_LEN, 0);

        let proof_extra_commitment =
            U256::from_be_slice(keccak256(prover.as_slice()).as_slice()) >> U256::from(8);

        // applying 10ETH limit
        let actual_balance = proof.balance;

        let intended_balance = if actual_balance > *BURN_AMOUNT_LIMIT {
            *BURN_AMOUNT_LIMIT
        } else {
            actual_balance
        };

        Ok(Self {
            actualBalance: actual_balance.to_string(),
            intendedBalance: intended_balance.to_string(),
            numLayers: proof.account_proof.len(),
            layerLens: layer_bits_lens,
            layers,
            blockHeader: extended_header,
            blockHeaderLen: header_bytes.len(),
            numLeafAddressNibbles: num_addr_hash_nibbles.to_string(),
            burnKey: burn_key.to_string(),
            revealAmount: spend.to_string(),
            burnExtraCommitment: burn_extra_commitment.to_string(),
            byteSecurityRelax: 0,
            _proofExtraCommitment: proof_extra_commitment.to_string(),
        })
    }

    pub fn to_json(&self) -> String {
        serde_json::to_string_pretty(self).unwrap()
    }

    pub fn write_to_file(&self, path: &PathBuf) -> Result<(), anyhow::Error> {
        fs::write(path, self.to_json())?;
        Ok(())
    }
}

// model for leaf of merkle patricia trie
#[derive(Debug, RlpDecodable, PartialEq)]
struct RlpLeaf {
    key: alloy::rlp::Bytes,
    value: alloy::rlp::Bytes,
}

#[derive(thiserror::Error, Debug)]
pub enum WitnessInputFileError {
    #[error("proof and block are not in same block")]
    NotOnSameBlock,

    #[error("unknown error: {0}")]
    Other(#[from] anyhow::Error),
}
