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
    Staking,
    "./src/contracts/abis/Staking.abi.json"
);

struct StakingContract {
    pub instance: StakingContractType,
}

impl StakingContract {
    pub async fn new(
        network: &str,
        address: Address,
        signer: PrivateKeySigner,
    ) -> Result<Self, TransportError> {
        let provider = ProviderBuilder::new()
            .wallet(signer)
            .connect(network)
            .await?;
        Ok(StakingContract {
            instance: Staking::new(address, provider),
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
