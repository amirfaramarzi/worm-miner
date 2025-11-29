use alloy::{
    primitives::{utils::parse_ether, *},
    signers::local::PrivateKeySigner,
};
use clap::{Parser, Subcommand};
use core::{burn::broadcaster::Broadcaster, contracts::network::Network};
use std::{path::PathBuf, str::FromStr};

/// Worm CLI
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Args {
    #[command(subcommand)]
    pub command: Commands,

    #[arg(long, default_value_t = Network::Mainnet, value_enum)]
    pub network: Network,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Burn ETH
    Burn {
        /// Account that preforms burn
        #[arg(long, value_parser = private_key_parser)]
        private_key: PrivateKeySigner,

        /// The amount we want to send to burn address
        #[arg(long, value_parser = eth_amount_parser)]
        amount: U256,

        /// Default is amount (maximum)
        #[arg(long, value_parser = eth_amount_parser)]
        reveal: Option<U256>,

        #[arg(long, value_parser = eth_amount_parser, default_value_t = { U256::from(0) })]
        broadcaster_fee: U256,

        // Amount of tokens you want to swap on uniswap
        #[arg(long, value_parser = eth_amount_parser, default_value_t = { U256::from(0) })]
        sell_on_uniswap: U256,

        /// User will get BETH on this address 0x...
        #[arg(long, value_parser = eth_address_parser)]
        receiver_address: Address,

        /// Default is `0` in case you want to prove it yourself
        #[arg(long, value_parser = eth_amount_parser, default_value_t = { U256::from(0) })]
        prover_fee: U256,
    },

    /// In case the proving/minting fails along the way, you can recover
    Recover {
        /// Json file (ex: burn.json)
        #[arg(long)]
        file: PathBuf,
    },

    /// Creates a new note file for the remaining amount (E.g note2.json)
    Spend {
        /// Note file (ex: note.json)
        #[arg(long)]
        note: PathBuf,

        #[arg(long, value_parser = eth_amount_parser)]
        amount: U256,
    },

    /// Put BETH to epochs to get Worm later,
    /// Creates `participate.json` to use in `claim` command
    Participate {
        /// Number of epochs you want to participate
        #[arg(long)]
        num_epochs: u64,

        /// How much you want to put in each epoch
        #[arg(long, value_parser = eth_amount_parser)]
        amount_per_epoch: U256,
    },

    /// Claim Worm form finished epoch
    Claim {
        /// participate.json file that created by Participate command
        participate_file: PathBuf,
    },
}

fn eth_amount_parser(s: &str) -> Result<U256, &'static str> {
    dbg!(&s);
    parse_ether(s).map_err(|_| "invalid eth amount")
}

fn eth_address_parser(s: &str) -> Result<Address, &'static str> {
    dbg!(&s);
    Address::from_str(s).map_err(|_| "invalid address")
}

fn broadcaster_parser(s: &str) -> Result<Broadcaster, &'static str> {
    dbg!(&s);
    Broadcaster::try_from(s).map_err(|_| "invalid broadcaster")
}

fn private_key_parser(s: &str) -> Result<PrivateKeySigner, &'static str> {
    dbg!(&s);
    PrivateKeySigner::from_str(s).map_err(|_| "invalid private key")
}
