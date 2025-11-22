use std::fmt::Display;

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
pub enum Network {
    Anvil,
    Sepolia,
    Mainnet,
}

impl TryFrom<&str> for Network {
    type Error = String;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Ok(match value {
            "anvil" => Network::Anvil,
            "sepolia" => Network::Sepolia,
            "mainnet" => Network::Mainnet,
            _ => return Err(format!("Invalid network: '{value}' ")),
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
    pub fn beth_address(&self) -> &'static str {
        match self {
            Network::Anvil => "",
            Network::Sepolia => "",
            Network::Mainnet => "",
        }
    }

    //TODO
    pub fn worm_address(&self) -> &'static str {
        match self {
            Network::Anvil => "",
            Network::Sepolia => "",
            Network::Mainnet => "",
        }
    }

    //TODO
    pub fn staking_address(&self) -> &'static str {
        match self {
            Network::Anvil => "",
            Network::Sepolia => "",
            Network::Mainnet => "",
        }
    }
}
