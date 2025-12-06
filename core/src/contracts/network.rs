use alloy::primitives::Address;
use anyhow::anyhow;
use clap::ValueEnum;
use std::{env, fmt::Display, str::FromStr};

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, ValueEnum)]
pub enum Network {
    Anvil,
    Sepolia,
    Mainnet,
}

impl TryFrom<&str> for Network {
    type Error = anyhow::Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Ok(match value {
            "anvil" => Network::Anvil,
            "sepolia" => Network::Sepolia,
            "mainnet" => Network::Mainnet,
            _ => return Err(anyhow!("Invalid network: '{value}' ")),
        })
    }
}

impl Display for Network {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Network::Anvil => f.write_str("anvil"),
            Network::Sepolia => f.write_str("sepolia"),
            Network::Mainnet => f.write_str("mainnet"),
        }
    }
}

impl Default for Network {
    fn default() -> Self {
        Network::Mainnet
    }
}

impl Network {
    pub fn url(&self) -> &'static str {
        match self {
            Network::Anvil => "http://127.0.0.1:8545",
            Network::Sepolia => "https://sepolia.drpc.org",
            Network::Mainnet => "https://mainnet.gateway.tenderly.co",
        }
    }

    //TODO
    pub fn beth_address(&self) -> Address {
        match self {
            Network::Anvil => {
                let s = env::var("ANVIL_BETH_ADDRESS")
                    .expect("provide ANVIL_BETH_ADDRESS env variable");
                Address::from_str(&s).unwrap()
            }
            Network::Sepolia => Address::from_str("todo").unwrap(),
            Network::Mainnet => Address::from_str("todo").unwrap(),
        }
    }

    //TODO
    pub fn worm_address(&self) -> String {
        match self {
            Network::Anvil => {
                env::var("ANVIL_WORM_ADDRESS").expect("provide ANVIL_WORM_ADDRESS env variable")
            }
            Network::Sepolia => "".to_string(),
            Network::Mainnet => "".to_string(),
        }
    }

    //TODO
    pub fn staking_address(&self) -> String {
        match self {
            Network::Anvil => env::var("ANVIL_STAKING_ADDRESS")
                .expect("provide ANVIL_STAKING_ADDRESS env variable"),
            Network::Sepolia => "".to_string(),
            Network::Mainnet => "".to_string(),
        }
    }
}
