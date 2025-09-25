pub mod initialize;
pub mod logging_set_level;
pub mod ping;
pub mod resources_list;

use serde::Deserialize;
use serde::Serialize;

use crate::mcp::jsonrpc::id::Id;

#[derive(Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Request<TPayload> {
    pub id: Id,
    pub jsonrpc: String,
    #[serde(flatten)]
    pub payload: TPayload,
}
