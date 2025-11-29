use alloy::{providers::PendingTransactionError, transports::TransportError};
use std::borrow::Cow;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum BurnError {
    #[error("invalid {what} = '{value}', {why}")]
    Validation {
        what: Cow<'static, str>,
        value: Cow<'static, str>,
        why: Cow<'static, str>,
    },

    #[error("unknown error: {0}")]
    Unknown(#[from] anyhow::Error),

    #[error("transport error: {0}")]
    Transport(#[from] TransportError),

    #[error("pending error: {0}")]
    Pending(#[from] PendingTransactionError),
}

impl BurnError {
    pub fn validation(
        what: impl Into<Cow<'static, str>>,
        value: impl Into<Cow<'static, str>>,
        why: impl Into<Cow<'static, str>>,
    ) -> Self {
        BurnError::Validation {
            what: what.into(),
            value: value.into(),
            why: why.into(),
        }
    }
}
