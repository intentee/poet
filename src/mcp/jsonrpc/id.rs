use serde::Deserialize;
use serde::Serialize;

#[derive(Debug, Deserialize, Serialize)]
#[serde(untagged)]
pub enum Id {
    Number(i32),
    String(String),
}
