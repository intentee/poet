use serde::Deserialize;
use serde::Serialize;

#[derive(Debug, Deserialize, Eq, PartialEq, Serialize)]
pub enum Role {
    #[serde(rename = "assistant")]
    Assistant,
    #[serde(rename = "user")]
    User,
}
