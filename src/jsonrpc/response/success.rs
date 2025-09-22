use serde::Deserialize;
use serde::Serialize;
use serde_json::Value;

use crate::jsonrpc::id::Id;

#[derive(Debug, Deserialize, Serialize)]
pub struct Success {
    pub id: Id,
    pub jsonrpc: String,
    pub result: Value,
}
