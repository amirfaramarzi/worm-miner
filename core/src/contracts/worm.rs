use alloy::{
    network::EthereumWallet,
    primitives::Address,
    providers::{ProviderBuilder, RootProvider, fillers::*},
    signers::local::PrivateKeySigner,
    sol,
    transports::TransportError,
};

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
    pub async fn new(
        rpc_url: &str,
        address: Address,
        signer: PrivateKeySigner,
    ) -> Result<Self, TransportError> {
        let provider = ProviderBuilder::new()
            .wallet(signer)
            .connect(rpc_url)
            .await?;
        Ok(WormContract {
            instance: Worm::new(address, provider),
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
