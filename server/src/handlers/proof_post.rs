use crate::{
    data::AppState,
    error::{LogIfError, ServerError},
    proof_queue_service::proof_job::ProofJob,
};
use alloy::{
    consensus::{BlockHeader, Header},
    eips::BlockNumberOrTag,
    primitives::{Address, Bytes, FixedBytes, U256, keccak256},
    providers::Provider,
    rpc::types::EIP1186AccountProofResponse,
};
use alloy_rlp::RlpEncodable;
use anyhow::anyhow;
use axum::{Json, extract::State, response::IntoResponse};
use common::contracts::network::Network;
use common::utils::ether_amount_serializer;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::sync::RwLock;

#[derive(Debug, RlpEncodable)]
struct MptLeaf {
    nonce: u64,
    balance: U256,
    code_hash: FixedBytes<32>,
    storage_hash: FixedBytes<32>,
}

pub fn check_validity(
    proof: EIP1186AccountProofResponse,
    header: Header,
) -> Result<(), anyhow::Error> {
    let state_root = header.state_root();
    if keccak256(&proof.account_proof[0]) != state_root {
        return Err(anyhow!("State-root inconsistency!"));
    }

    for i in 1..proof.account_proof.len() {
        let hash = keccak256(&proof.account_proof[i]);
        if !proof.account_proof[i - 1].windows(32).any(|w| w == hash) {
            return Err(anyhow!("Layer inconsistency!"));
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
    let key_val: Vec<Bytes> = alloy::rlp::decode_exact(&last_layer)?;
    if key_val.len() != 2 {
        return Err(anyhow!("Weird leaf!"));
    }

    if key_val[1] != leaf_rlp {
        return Err(anyhow!("Account inconsistency!"));
    }

    let key_hex = if key_val[0][0] & 0xf0 == 0x30 {
        String::from(&format!("{:x}", key_val[0][0])[1..])
    } else if key_val[0][0] == 0x20 {
        String::from(&format!("{:x}", key_val[0][0])[2..])
    } else {
        return Err(anyhow!("Weird address hash prefix!"));
    };

    if !format!("{:x}", addr_hash).ends_with(&key_hex) {
        return Err(anyhow!("Address inconsistency!"));
    }

    Ok(())
}

/// POST `/proof` gets inputs of the proof-of-burn zk circuit and starts proving.
pub async fn proof_post(
    State(state): State<Arc<RwLock<AppState>>>,
    Json(body): Json<ProofPostRequest>,
) -> Result<ProofPostResponse, ServerError> {
    let mut state = state.write().await;
    if !state.header_cache.contains_key(&body.target_block) {
        let header = state
            .provider
            .get_block_by_number(BlockNumberOrTag::Number(body.target_block))
            .await?
            .ok_or(anyhow!("Block not found!"))?
            .header;
        state.header_cache.insert(body.target_block, header.into());
    }

    let block_header = state
        .header_cache
        .get(&body.target_block)
        .cloned()
        .ok_or(anyhow!("Header not found!"))?;

    check_validity(body.account_proof.clone(), block_header.clone())?;

    let job = ProofJob::new(body, block_header);
    if let Err(e) = state.job_channel.send(job) {
        return Err(ServerError::Unexpected(
            anyhow!("{e}").into_boxed_dyn_error(),
        ))
        .log_with_context("send_job_to_channel");
    }

    Ok(ProofPostResponse {})
}

#[derive(Deserialize)]
pub struct ProofPostRequest {
    pub target_block: u64,
    pub account_proof: EIP1186AccountProofResponse,

    pub network: Network,
    pub burn_key: U256,
    pub receiver_address: Address,

    #[serde(with = "ether_amount_serializer")]
    pub broadcaster_fee: U256,
    #[serde(with = "ether_amount_serializer")]
    pub prover_fee: U256,
    #[serde(with = "ether_amount_serializer")]
    pub spend: U256,

    pub receiver_hook: Bytes,
}

#[derive(Serialize)]
pub struct ProofPostResponse {}

impl IntoResponse for ProofPostResponse {
    fn into_response(self) -> axum::response::Response {
        Json(self).into_response()
    }
}
