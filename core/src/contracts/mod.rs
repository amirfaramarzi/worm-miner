pub mod beth;
pub mod network;
pub mod staking;
pub mod uniswap;
pub mod worm;

use std::{error::Error, str::FromStr};

use alloy::{primitives::Address, signers::local::PrivateKeySigner};
use beth::*;
use network::*;
use staking::*;
use worm::*;

pub struct Contracts {
    beth: BETHContract,
    staking: StakingContract,
    worm: WormContract,
}

impl Contracts {
    pub async fn new(
        network: impl AsRef<str>,
        private_key: impl AsRef<str>,
    ) -> Result<Self, anyhow::Error> {
        let network = Network::try_from(network.as_ref())?;
        let signer: PrivateKeySigner = private_key.as_ref().parse()?;

        let rpc_url = network.url();
        let beth_addr = Address::from_str(&network.beth_address())?;
        let staking_addr = Address::from_str(&network.staking_address())?;
        let worm_addr = Address::from_str(&network.worm_address())?;
        Ok(Self {
            beth: BETHContract::new(rpc_url, beth_addr, signer.clone()).await?,
            staking: StakingContract::new(rpc_url, staking_addr, signer.clone()).await?,
            worm: WormContract::new(rpc_url, worm_addr, signer).await?,
        })
    }

    pub fn beth(&self) -> &BETHContract {
        &self.beth
    }

    pub fn staking(&self) -> &StakingContract {
        &self.staking
    }

    pub fn worm(&self) -> &WormContract {
        &self.worm
    }
}
