use crate::arg_parser::Args;
use clap::Parser;
pub mod arg_parser;

fn main() {
    let args = Args::parse();

    println!("args: {:?}", args);
}
