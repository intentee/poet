use serde::Deserialize;
use serde::Serialize;

use crate::jsonrpc_version::JSONRPC_VERSION;
use crate::request_id::RequestId;

#[derive(Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct JsonRpcSuccessResponse<TResult> {
    pub id: RequestId,
    pub jsonrpc: String,
    pub result: TResult,
}

impl<TResult> JsonRpcSuccessResponse<TResult> {
    pub fn new(id: RequestId, result: TResult) -> Self {
        Self {
            id,
            jsonrpc: JSONRPC_VERSION.to_owned(),
            result,
        }
    }
}
