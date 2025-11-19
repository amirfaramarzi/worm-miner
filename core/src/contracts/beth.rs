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
    BETH,
    "./src/contracts/abis/BETH.abi.json"
);

struct BETHContract {
    pub instance: BETHContractType,
}

impl BETHContract {
    pub async fn new(
        network: &str,
        address: Address,
        signer: PrivateKeySigner,
    ) -> Result<Self, TransportError> {
        let provider = ProviderBuilder::new()
            .wallet(signer)
            .connect(network)
            .await?;
        Ok(BETHContract {
            instance: BETH::new(address, provider),
        })
    }
}

type BETHContractType = BETH::BETHInstance<
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
