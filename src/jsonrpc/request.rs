use serde::Deserialize;
use serde::Serialize;
use serde_json::Value;

#[derive(Deserialize, Serialize)]
pub struct Request {
    pub id: String,
    pub jsonrpc: String,
    pub method: String,
    pub params: Value,
}
