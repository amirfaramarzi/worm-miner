use std::borrow::Cow;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum MintError {
    #[error("invalid {what} = '{value}', {why}")]
    Validation {
        what: Cow<'static, str>,
        value: Cow<'static, str>,
        why: Cow<'static, str>,
    },

    #[error("unknown error: {0}")]
    Unknown(#[from] anyhow::Error),
}

impl MintError {
    pub fn validation(
        what: impl Into<Cow<'static, str>>,
        value: impl Into<Cow<'static, str>>,
        why: impl Into<Cow<'static, str>>,
    ) -> Self {
        MintError::Validation {
            what: what.into(),
            value: value.into(),
            why: why.into(),
        }
    }
}
