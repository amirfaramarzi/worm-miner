pub mod arg_parser;
pub mod fs;

use crate::{arg_parser::Args, fs::*};
use clap::Parser;
use core::burn::burn;
use std::process::exit;

#[tokio::main]
async fn main() {
    let args = Args::parse();

    let network = args.network;
    match args.command {
        arg_parser::Commands::Burn {
            private_key,
            amount,
            reveal,
            broadcaster_fee,
            sell_on_uniswap,
            receiver_address,
            prover_fee,
            out: out_file,
        } => {
            // We should validate input file before start burning
            let out_file = match validate_output_file(out_file) {
                Ok(x) => x,
                Err(e) => {
                    eprintln!("invalid out file: {}", e);
                    exit(1);
                }
            };
            let out = match burn(
                network,
                private_key,
                amount,
                reveal.unwrap_or(amount),
                broadcaster_fee,
                sell_on_uniswap,
                receiver_address,
                prover_fee,
            )
            .await
            {
                Ok(x) => x,
                Err(e) => {
                    eprintln!("burn command failed: {}", e);
                    exit(1);
                }
            };
            let json = BurnOutputJson::from(out).to_json();
            if let Err(e) = std::fs::write(out_file, &json) {
                println!(
                    "error `{}` while writing to file, your burn.json here \n{}",
                    e, json
                )
            }
        }
        arg_parser::Commands::Recover { file: _ } => todo!(),
        arg_parser::Commands::Spend {
            note: _,
            amount: _amount,
        } => todo!(),
        arg_parser::Commands::Participate {
            num_epochs: _,
            amount_per_epoch: _,
        } => todo!(),
        arg_parser::Commands::Claim {
            participate_file: _,
        } => todo!(),
    };
}
