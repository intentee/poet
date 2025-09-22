use serde::Deserialize;
use serde::Serialize;
use serde_json::Value;

use crate::jsonrpc::id::Id;

#[derive(Debug, Deserialize, Serialize)]
pub struct Request {
    pub id: Id,
    pub jsonrpc: String,
    pub method: String,
    pub params: Value,
}
