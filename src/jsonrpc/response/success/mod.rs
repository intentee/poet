pub mod empty_response;
pub mod initialize_result;

use serde::Deserialize;
use serde::Serialize;

use crate::jsonrpc::id::Id;

#[derive(Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Success<TResult> {
    pub id: Id,
    pub jsonrpc: String,
    pub result: TResult,
}
