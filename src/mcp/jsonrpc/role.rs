use anyhow::Result;
use anyhow::anyhow;
use serde::Deserialize;
use serde::Serialize;

#[derive(Debug, Deserialize, Eq, PartialEq, Serialize)]
pub enum Role {
    #[serde(rename = "assistant")]
    Assistant,
    #[serde(rename = "user")]
    User,
}

impl TryFrom<&str> for Role {
    type Error = anyhow::Error;

    fn try_from(value: &str) -> Result<Self> {
        match value {
            "assistant" => Ok(Role::Assistant),
            "user" => Ok(Role::User),
            _ => Err(anyhow!("Unknown role: {value}")),
        }
    }
}
