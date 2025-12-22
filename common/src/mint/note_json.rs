use crate::contracts::network::Network;
use alloy::primitives::U256;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct NoteJson {
    burn_key: U256,
    amount: U256,
    network: Network,
}

impl NoteJson {
    pub fn new(burn_key: U256, amount: U256, network: Network) -> Self {
        Self {
            burn_key,
            amount,
            network,
        }
    }

    pub fn from_json(s: &str) -> Result<Self, anyhow::Error> {
        Ok(serde_json::from_str(s)?)
    }

    pub fn to_json(&self) -> Result<String, anyhow::Error> {
        Ok(serde_json::to_string_pretty(self)?)
    }

    pub fn burn_key(&self) -> U256 {
        self.burn_key
    }

    pub fn amount(&self) -> U256 {
        self.amount
    }

    pub fn network(&self) -> Network {
        self.network
    }
}
