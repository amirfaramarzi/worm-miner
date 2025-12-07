use crate::contracts::network::Network;
use alloy::primitives::{Address, Bytes, U256};

#[derive(serde::Deserialize, serde::Serialize)]
pub struct BurnOutput {
    // So we don't need to get it from user again on burn
    pub network: Network,

    // Burn_address 2nd param
    pub burn_key: U256, // Save as string because it's a pretty large number

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
        burn_key: U256,
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

    pub fn from_json(s: &str) -> Result<Self, anyhow::Error> {
        Ok(serde_json::from_str(s)?)
    }

    pub fn to_json(&self) -> Result<String, anyhow::Error> {
        Ok(serde_json::to_string_pretty(self)?)
    }
}
