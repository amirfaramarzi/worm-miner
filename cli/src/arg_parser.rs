use clap::{Parser, Subcommand};

/// Worm CLI
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Args {
    #[command(subcommand)]
    command: Commands,

    /// Options: ("anvil" | "sepolia" | "mainnet")
    /// default: mainnet
    #[arg(long)]
    network: Option<String>,
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
    },
}
