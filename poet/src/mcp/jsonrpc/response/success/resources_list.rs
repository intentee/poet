use serde::Deserialize;
use serde::Serialize;

use crate::mcp::list_resources_cursor::ListResourcesCursor;
use crate::mcp::resource::Resource;

#[derive(Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct ResourcesList {
    #[serde(
        default,
        rename = "nextCursor",
        skip_serializing_if = "Option::is_none",
        with = "crate::mcp::list_resources_cursor"
    )]
    pub next_cursor: Option<ListResourcesCursor>,
    pub resources: Vec<Resource>,
}
