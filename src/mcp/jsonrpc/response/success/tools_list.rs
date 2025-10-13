use serde::Deserialize;
use serde::Serialize;

use crate::mcp::tool::Tool;

#[derive(Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct ToolsList {
    pub tools: Vec<Tool>,
}
