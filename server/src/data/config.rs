use alloy::primitives::Address;
use alloy::primitives::utils::format_ether;
use alloy::primitives::{U256, utils::parse_ether};
use common::utils::ether_amount_serializer;
use common::utils::worm_home;
use serde::{Deserialize, Serialize};
use std::fs;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Config {
    #[serde(with = "ether_amount_serializer")]
    pub min_prover_fee: U256,
    #[serde(with = "ether_amount_serializer")]
    pub min_broadcaster_fee: U256,
    pub owner_address: Address,
    pub port: u16,
}

impl Config {
    pub fn load() -> Result<Config, anyhow::Error> {
        let path = worm_home::get_config()?;
        if !path.exists() {
            println!("server config not exist!");
            Self::create_default_file()?;
            println!("server config created at '{}'", path.to_str().unwrap());
        }
        Ok(serde_json::from_str(&fs::read_to_string(path)?)?)
    }

    pub fn save(&self) -> Result<(), anyhow::Error> {
        fs::write(worm_home::get_config()?, self.to_json()?)?;
        Ok(())
    }

    pub fn create_default_file() -> Result<(), anyhow::Error> {
        Config::default().save()
    }

    pub fn to_json(&self) -> Result<String, anyhow::Error> {
        Ok(serde_json::to_string_pretty(self)?)
    }

    pub fn print(&self) {
        // this prevent user confusion by specifying units (ETH)
        println!(
            "\n--- Loading config from `{}` ",
            worm_home::get_config().unwrap().to_str().unwrap()
        );
        println!(
            "min_prover_fee      = {} ETH",
            format_ether(self.min_prover_fee)
                .trim_end_matches('0')
                .trim_end_matches('.')
        );
        println!(
            "min_broadcaster_fee = {} ETH",
            format_ether(self.min_broadcaster_fee)
                .trim_end_matches('0')
                .trim_end_matches('.')
        );
        println!("owner address       = {}", self.owner_address);
        println!("port                = {}", self.port);

        if self.owner_address == Address::ZERO {
            println!(
                " --- WARNING!! ---\nowner address is ZERO make sure you change it to you address"
            )
        }
        println!()
    }
}

impl Default for Config {
    fn default() -> Self {
        Self {
            min_prover_fee: parse_ether("0.001").unwrap(),
            min_broadcaster_fee: parse_ether("0.001").unwrap(),
            port: 8080,
            owner_address: Address::ZERO,
        }
    }
}
