use serde::Deserialize;
use serde::Serialize;

use crate::meta::Meta;

#[derive(Debug, Default, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct ResourcesTemplatesListRequestParams {
    #[serde(rename = "_meta", skip_serializing_if = "Option::is_none")]
    pub meta: Option<Meta>,
}
