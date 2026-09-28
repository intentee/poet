use std::str::FromStr;

use serde::Deserialize;
use serde::Serialize;
use serde::de::IntoDeserializer as _;
use serde::de::value::Error as DeserializationError;

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub enum Role {
    #[serde(rename = "assistant")]
    Assistant,
    #[serde(rename = "user")]
    User,
}

impl FromStr for Role {
    type Err = DeserializationError;

    fn from_str(role_name: &str) -> Result<Self, Self::Err> {
        Self::deserialize(role_name.into_deserializer())
    }
}
