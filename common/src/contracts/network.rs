use alloy::primitives::Address;
use anyhow::anyhow;
use clap::ValueEnum;
use std::{env, fmt::Display, str::FromStr};

#[derive(
    Clone,
    Copy,
    Debug,
    Hash,
    PartialEq,
    Eq,
    ValueEnum,
    serde::Serialize,
    serde::Deserialize,
    Default,
)]
#[serde(rename_all = "lowercase")]
pub enum Network {
    Anvil,
    Sepolia,
    #[default]
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

impl Network {
    pub fn url(&self) -> &'static str {
        match self {
            Network::Anvil => "http://127.0.0.1:8545",
            Network::Sepolia => "https://sepolia.drpc.org",
            Network::Mainnet => "https://mainnet.gateway.tenderly.co",
        }
    }

    //TODO
    pub fn beth_address(&self) -> Result<Address, anyhow::Error> {
        let address_str = match self {
            Network::Anvil => {
                env::var("ANVIL_BETH_ADDRESS").expect("provide ANVIL_BETH_ADDRESS env variable")
            }
            Network::Sepolia => "0x1a0cb44C7DcB767DD6f819DA0Ae16485B57C4738".to_string(),
            Network::Mainnet => "todo".to_string(),
        };
        Address::from_str(&address_str)
            .map_err(|e| anyhow!("invalid beth contract address {}, msg: {}", address_str, e))
    }

    //TODO
    pub fn worm_address(&self) -> Result<Address, anyhow::Error> {
        let address_str = match self {
            Network::Anvil => {
                env::var("ANVIL_WORM_ADDRESS").expect("provide ANVIL_WORM_ADDRESS env variable")
            }
            Network::Sepolia => "0xC5BDeF279Ec3412c48C29239c43C292355D81144".to_string(),
            Network::Mainnet => "todo".to_string(),
        };
        Address::from_str(&address_str)
            .map_err(|e| anyhow!("invalid worm contract address {}, msg: {}", address_str, e))
    }

    //TODO
    pub fn staking_address(&self) -> Result<Address, anyhow::Error> {
        let address_str = match self {
            Network::Anvil => env::var("ANVIL_STAKING_ADDRESS")
                .expect("provide ANVIL_STAKING_ADDRESS env variable"),
            Network::Sepolia => "0x0AF06bBE75a98B0062E67D4f49442cf73fA17586".to_string(),
            Network::Mainnet => "todo".to_string(),
        };
        Address::from_str(&address_str).map_err(|e| {
            anyhow!(
                "invalid staking contract address {}, msg: {}",
                address_str,
                e
            )
        })
    }
}
