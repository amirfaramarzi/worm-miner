use alloy::primitives::{
    U256,
    utils::{format_ether, parse_ether},
};
use serde::{Deserialize, Deserializer, Serializer};

pub fn serialize<S>(value: &U256, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    serializer.serialize_str(
        &format_ether(*value)
            .trim_end_matches('0')
            .trim_end_matches('.'),
    )
}

pub fn deserialize<'de, D>(deserializer: D) -> Result<U256, D::Error>
where
    D: Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;
    parse_ether(&s)
        .map_err(|e| serde::de::Error::custom(format!("invalid ether amount: {}: {}", s, e)))
}
