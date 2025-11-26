use clap::{Parser, Subcommand};
use core::contracts::network::Network;

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
        #[arg(long)]
        private_key: String,

        /// The amount we want to send to burn address
        #[arg(long)]
        amount: u64,

        /// Default is amount (maximum)
        #[arg(long)]
        reveal: Option<u64>,

        #[arg(long, default_value_t = 0)]
        broadcaster_fee: u64,

        /// Endpoint or different private-key
        #[arg(long)]
        broadcaster: String,

        #[arg(long, default_value_t = 0)]
        sell_on_uniswap: u64,

        /// User will get BETH on this address 0x...
        #[arg(long)]
        receiver_address: String,

        /// Default is `0` in case you want to prove it yourself
        #[arg(long, default_value_t = 0)]
        prover_fee: u64,
    },

    /// In case the proving/minting fails along the way, you can recover
    Recover {
        /// Json file (ex: burn.json)
        #[arg(long)]
        file: String,
    },

    /// Creates a new note file for the remaining amount (E.g note2.json)
    Spend {
        /// Note file (ex: note.json)
        #[arg(long)]
        note: String,

        #[arg(long)]
        amount: u64,
    },

    /// Put BETH to epochs to get Worm later,
    /// Creates `participate.json` to use in `claim` command
    Participate {
        /// Number of epochs you want to participate
        #[arg(long)]
        num_epochs: u64,

        /// How much you want to put in each epoch
        #[arg(long)]
        amount_per_epoch: u64,
    },

    /// Claim Worm form finished epoch
    Claim {
        /// participate.json file that created by Participate command
        participate_file: String,
    },
}
