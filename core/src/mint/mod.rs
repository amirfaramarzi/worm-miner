pub mod error;

use crate::{
    burn::{broadcaster::Broadcaster, burn_output::BurnOutput},
    mint::error::MintError,
};

pub async fn mint(burn_output: BurnOutput, broadcaster: Broadcaster) -> Result<(), MintError> {
    todo!()
}
