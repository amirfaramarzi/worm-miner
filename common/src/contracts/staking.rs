use alloy::{
    network::EthereumWallet,
    providers::{ProviderBuilder, RootProvider, fillers::*},
    signers::local::PrivateKeySigner,
    sol,
};

use crate::contracts::network::Network;

sol!(
    #[allow(missing_docs)]
    #[sol(rpc)]
    Staking,
    "./src/contracts/abis/Staking.abi.json"
);

pub struct StakingContract {
    pub instance: StakingContractType,
}

impl StakingContract {
    pub async fn new(network: Network, signer: PrivateKeySigner) -> Result<Self, anyhow::Error> {
        let provider = ProviderBuilder::new()
            .wallet(signer)
            .connect(network.url())
            .await?;
        Ok(StakingContract {
            instance: Staking::new(network.staking_address()?, provider),
        })
    }
}

type StakingContractType = Staking::StakingInstance<
    FillProvider<
        JoinFill<
            JoinFill<
                alloy::providers::Identity,
                JoinFill<GasFiller, JoinFill<BlobGasFiller, JoinFill<NonceFiller, ChainIdFiller>>>,
            >,
            WalletFiller<EthereumWallet>,
        >,
        RootProvider,
    >,
>;
