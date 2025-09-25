use serde::Deserialize;
use serde::Serialize;

use crate::mcp::jsonrpc::id::Id;

#[derive(Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Meta {
    #[serde(rename = "progressToken", skip_serializing_if = "Option::is_none")]
    pub progress_token: Option<Id>,
}
