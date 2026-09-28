use serde::Deserialize;
use serde::Serialize;

use crate::logging_set_level_request_params::LoggingSetLevelRequestParams;
use crate::request_id::RequestId;

#[derive(Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct LoggingSetLevelRequest {
    pub id: RequestId,
    pub jsonrpc: String,
    pub params: LoggingSetLevelRequestParams,
}
