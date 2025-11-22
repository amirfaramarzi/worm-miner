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

pub struct BETHContract {
    pub instance: BETHContractType,
}

impl BETHContract {
    pub async fn new(
        rpc_url: &str,
        address: Address,
        signer: PrivateKeySigner,
    ) -> Result<Self, TransportError> {
        let provider = ProviderBuilder::new()
            .wallet(signer)
            .connect(rpc_url)
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
