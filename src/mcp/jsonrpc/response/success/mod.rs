pub mod empty_response;
pub mod initialize_result;
pub mod resource_templates_list;
pub mod resources_list;
pub mod resources_read;

use serde::Deserialize;
use serde::Serialize;

use crate::mcp::jsonrpc::id::Id;

#[derive(Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Success<TResult> {
    pub id: Id,
    pub jsonrpc: String,
    pub result: TResult,
}
