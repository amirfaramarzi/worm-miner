use std::{borrow::Cow, error::Error, fmt::Display};

use alloy::transports::{RpcError, TransportErrorKind};
use axum::{
    http::{HeaderMap, StatusCode, header},
    response::IntoResponse,
};

#[derive(thiserror::Error, Debug)]
pub enum ServerError {
    #[error("{0}")]
    Validation(ValidationError),

    #[error("{0} Not found")]
    NotFound(String),

    #[error("Invalid action: {0}")]
    InvalidAction(&'static str),

    #[error("Unexpected error: {0}")]
    Unexpected(#[from] Box<dyn Error>),

    #[error("RPC transport error: {0}")]
    RpcTransport(#[from] RpcError<TransportErrorKind>),
}

impl From<anyhow::Error> for ServerError {
    fn from(value: anyhow::Error) -> Self {
        Self::Unexpected(value.into_boxed_dyn_error())
    }
}

impl ServerError {
    pub fn validation(
        what: impl Into<Cow<'static, str>>,
        value: impl Into<Cow<'static, str>>,
        why: impl Into<Cow<'static, str>>,
    ) -> Self {
        Self::Validation(ValidationError {
            what: what.into(),
            value: value.into(),
            why: why.into(),
        })
    }
}

#[derive(Debug, Clone)]
pub struct ValidationError {
    what: Cow<'static, str>,
    value: Cow<'static, str>,
    why: Cow<'static, str>,
}

impl Display for ValidationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&format!(
            "invalid {} = '{}', {}",
            self.what, self.value, self.why
        ))
    }
}

impl IntoResponse for ServerError {
    fn into_response(self) -> axum::response::Response {
        let mut error_message = self.to_string();

        let status = match self {
            ServerError::Unexpected(_) => {
                error_message = "unknown error".to_string(); // prevent information leak
                StatusCode::INTERNAL_SERVER_ERROR
            }
            ServerError::Validation(_) => StatusCode::BAD_REQUEST,
            ServerError::NotFound(_) => StatusCode::NOT_FOUND,
            ServerError::InvalidAction(_) => StatusCode::FORBIDDEN,
            ServerError::RpcTransport(_) => StatusCode::INTERNAL_SERVER_ERROR,
        };

        let mut headers = HeaderMap::new();
        headers.insert(header::CONTENT_TYPE, "application/json".parse().unwrap());

        (
            status,
            headers,
            serde_json::json!({
                "error": error_message,
            })
            .to_string(),
        )
            .into_response()
    }
}

// --- Logging ---

impl ServerError {
    pub fn log(self) -> Self {
        println!("error: {self}");
        self
    }

    fn log_with_context(self, context: &'static str) -> Self {
        println!("context: '{context}' error: {self}");
        self
    }
}

pub trait LogIfError {
    fn log(self) -> Self;
    fn log_with_context(self, context: &'static str) -> Self;
}

impl<T> LogIfError for Result<T, ServerError> {
    fn log(self) -> Self {
        if let Err(e) = &self {
            println!("error: {e}");
        }
        self
    }

    fn log_with_context(self, context: &'static str) -> Self {
        if let Err(e) = &self {
            println!("context: '{context}' error: {e}");
        }
        self
    }
}
