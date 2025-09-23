use serde::Deserialize;
use serde::Serialize;

use crate::jsonrpc::id::Id;

#[derive(Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct ParamsMeta {
    #[serde(rename = "progressToken", skip_serializing_if = "Option::is_none")]
    pub progress_token: Option<Id>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct ParamsWithMeta<TParams> {
    #[serde(flatten)]
    pub params: TParams,
    #[serde(rename = "_meta", skip_serializing_if = "Option::is_none")]
    pub meta: Option<ParamsMeta>,
}
