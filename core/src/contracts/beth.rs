use crate::{contracts::network::Network, mint::proof_generator::RapidsnarkOutput};
use alloy::{
    network::EthereumWallet,
    primitives::{Address, Bytes, U256},
    providers::{ProviderBuilder, RootProvider, fillers::*},
    rpc::types::TransactionReceipt,
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
    pub async fn new(network: Network, signer: PrivateKeySigner) -> Result<Self, TransportError> {
        let provider = ProviderBuilder::new()
            .wallet(signer)
            .connect(network.url())
            .await?;
        Ok(BETHContract {
            instance: BETH::new(network.beth_address(), provider),
        })
    }

    pub async fn mint(
        &self,
        proof: RapidsnarkOutput,
        block_number: U256,
        nullifier: U256,
        remaining_coin_hash: U256,
        broadcaster_fee: U256,
        spend: U256,
        receiver: Address,
        prover_fee: U256,
        prover: Address,
        swap_calldata: Bytes,
    ) -> Result<TransactionReceipt, anyhow::Error> {
        let receipt = self
            .instance
            .mintCoin(
                // pi_a
                [proof.proof.pi_a[0], proof.proof.pi_a[1]],
                // pi_b (flipped coordinates)
                [
                    [proof.proof.pi_b[0][1], proof.proof.pi_b[0][0]],
                    [proof.proof.pi_b[1][1], proof.proof.pi_b[1][0]],
                ],
                // pi_c
                [proof.proof.pi_c[0], proof.proof.pi_c[1]],
                block_number,
                nullifier,
                remaining_coin_hash,
                broadcaster_fee,
                spend,
                receiver,
                prover_fee,
                prover,
                swap_calldata,
                Bytes::new(),
            )
            .send()
            .await?
            .get_receipt()
            .await?;
        Ok(receipt)
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
