use serde::Deserialize;
use serde::Serialize;

use crate::mcp::resource::Resource;

#[derive(Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct ResourcesList {
    pub resources: Vec<Resource>,
}
