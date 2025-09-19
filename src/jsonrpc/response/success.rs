use serde::Deserialize;
use serde::Serialize;
use serde_json::Value;

#[derive(Deserialize, Serialize)]
pub struct Success {
    pub id: String,
    pub jsonrpc: String,
    pub result: Value,
}
