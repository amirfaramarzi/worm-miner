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

pub struct StakingContract {
    pub instance: StakingContractType,
}

impl StakingContract {
    pub async fn new(
        rpc_url: &str,
        address: Address,
        signer: PrivateKeySigner,
    ) -> Result<Self, TransportError> {
        let provider = ProviderBuilder::new()
            .wallet(signer)
            .connect(rpc_url)
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
