use alloy::primitives::U256;

/// TODO change that
pub type ProofResult = ();

pub struct ProofJob {
    pub nullifier: U256,
}

impl ProofJob {
    pub fn new(nullifier: U256) -> Self {
        Self { nullifier }
    }
}

impl ProofJob {
    pub async fn run(&self) -> ProofResult {
        // TODO
        ()
    }
}
