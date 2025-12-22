use crate::burn::error::BurnError;
use alloy::signers::local::PrivateKeySigner;
use reqwest::Url;
use std::str::FromStr;

#[derive(Clone, Debug)]
pub enum Broadcaster {
    EndPoint(Url),
    PrivateKey(PrivateKeySigner),
}

impl TryFrom<&str> for Broadcaster {
    type Error = BurnError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        if value.starts_with("http") {
            Ok(Broadcaster::EndPoint(Url::parse(value).map_err(|e| {
                BurnError::validation("broadcaster", (&value).to_string(), e.to_string())
            })?))
        } else if value.starts_with("0x") {
            Ok(Broadcaster::PrivateKey(
                PrivateKeySigner::from_str(value).map_err(|e| {
                    BurnError::validation("broadcaster", (&value).to_string(), e.to_string())
                })?,
            ))
        } else {
            Err(BurnError::validation(
                "broadcaster",
                (&value).to_string(),
                "should start with http for endpoint and 0x for private key",
            ))
        }
    }
}
