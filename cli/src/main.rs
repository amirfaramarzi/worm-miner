use core::burn::burn;

use crate::arg_parser::Args;
use clap::Parser;
pub mod arg_parser;

#[tokio::main]
async fn main() {
    let args = Args::parse();

    let network = args.network;
    let result = match args.command {
        arg_parser::Commands::Burn {
            private_key,
            amount,
            reveal,
            broadcaster_fee,
            broadcaster,
            sell_on_uniswap,
            receiver_address,
            prover_fee,
        } => {
            burn(
                network,
                private_key,
                amount,
                reveal.unwrap_or(amount),
                broadcaster_fee,
                broadcaster,
                sell_on_uniswap,
                receiver_address,
                prover_fee,
            )
            .await
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

    println!("--- task finished! ---");
    if let Err(e) = result {
        println!("Failed with error: \n{}", e)
    } else {
        println!("Successful")
    }
}
