use serde::Deserialize;
use serde::Serialize;

use crate::list_resources_cursor::ListResourcesCursor;
use crate::resource::Resource;

#[derive(Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct ResourcesListResult {
    #[serde(
        default,
        rename = "nextCursor",
        skip_serializing_if = "Option::is_none"
    )]
    pub next_cursor: Option<ListResourcesCursor>,
    pub resources: Vec<Resource>,
}
