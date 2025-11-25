use thiserror::Error;

#[derive(Error, Debug)]
pub enum BurnError {
    #[error("invalid {what} = '{value}', {why}")]
    Validation {
        what: &'static str,
        value: String,
        why: &'static str,
    },

    #[error("unknown error: {0}")]
    Unknown(#[from] anyhow::Error),
}
