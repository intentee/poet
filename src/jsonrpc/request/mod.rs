pub mod initialize;
pub mod logging_set_level;
pub mod ping;

use serde::Deserialize;
use serde::Serialize;

use crate::jsonrpc::id::Id;

#[derive(Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Request<TPayload> {
    pub id: Id,
    pub jsonrpc: String,
    #[serde(flatten)]
    pub payload: TPayload,
}
