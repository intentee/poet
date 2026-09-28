use serde::Deserialize;
use serde::Serialize;

use crate::list_resources_cursor::ListResourcesCursor;
use crate::meta::Meta;

#[derive(Debug, Default, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct ResourcesListRequestParams {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cursor: Option<ListResourcesCursor>,
    #[serde(rename = "_meta", skip_serializing_if = "Option::is_none")]
    pub meta: Option<Meta>,
}
