use alloy::primitives::{Address, Bytes, U256};
use anyhow::anyhow;
use ark_bn254::Fr;
use ark_ff::{BigInt, PrimeField};
use core::{burn::burn_output::BurnOutput, contracts::network::Network};
use std::str::FromStr;

/// Json friendly version of [core::BurnOutput]
#[derive(serde::Deserialize, serde::Serialize)]
pub struct BurnOutputJson {
    pub network: String,
    pub burn_key: String,
    pub reveal_amount: String,
    pub receiver: String,
    pub prover_fee: String,
    pub broadcaster_fee: String,
    pub receiver_hook: String,
}

impl BurnOutputJson {
    pub fn to_json(&self) -> String {
        serde_json::to_string_pretty(self).unwrap()
    }

    pub fn from_json(json: &str) -> Result<BurnOutputJson, serde_json::Error> {
        serde_json::from_str::<BurnOutputJson>(json)
    }
}

impl From<BurnOutput> for BurnOutputJson {
    fn from(value: BurnOutput) -> Self {
        BurnOutputJson {
            network: value.network.to_string(),
            burn_key: value.burn_key.to_string(),
            reveal_amount: value.reveal_amount.to_string(),
            receiver: value.receiver.to_string(),
            prover_fee: value.prover_fee.to_string(),
            broadcaster_fee: value.broadcaster_fee.to_string(),
            receiver_hook: value.receiver_hook.to_string(),
        }
    }
}

impl TryFrom<BurnOutputJson> for BurnOutput {
    type Error = anyhow::Error;
    fn try_from(value: BurnOutputJson) -> Result<Self, Self::Error> {
        Ok(BurnOutput::new(
            Network::try_from(value.network.as_str())?,
            Fr::from_bigint(
                BigInt::from_str(&value.burn_key).map_err(|_| anyhow!("burn_key bigint parse"))?,
            )
            .ok_or(anyhow!("burn_key Fr parse"))?,
            U256::from_str_radix(&value.reveal_amount, 10)?,
            Address::from_str(&value.receiver)?,
            U256::from_str_radix(&value.prover_fee, 10)?,
            U256::from_str_radix(&value.broadcaster_fee, 10)?,
            Bytes::from_str(&value.receiver_hook)?,
        ))
    }
}
