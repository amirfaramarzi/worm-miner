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
    Worm,
    "./src/contracts/abis/WORM.abi.json"
);

pub struct WormContract {
    pub instance: WormContractType,
}

impl WormContract {
    pub async fn new(network: Network, signer: PrivateKeySigner) -> Result<Self, anyhow::Error> {
        let provider = ProviderBuilder::new()
            .wallet(signer)
            .connect(network.url())
            .await?;
        Ok(WormContract {
            instance: Worm::new(network.worm_address()?, provider),
        })
    }
}

type WormContractType = Worm::WormInstance<
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
