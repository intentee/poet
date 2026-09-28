use serde::Deserialize;
use serde::Serialize;

use crate::tool::Tool;

#[derive(Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct ToolsListResult {
    pub tools: Vec<Tool>,
}
