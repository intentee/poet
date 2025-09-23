use serde::Deserialize;
use serde::Serialize;

#[derive(Debug, Deserialize, Serialize)]
pub struct ParamsMeta {}

#[derive(Debug, Deserialize, Serialize)]
// #[serde(deny_unknown_fields)]
pub struct ParamsWithMeta<TParams> {
    #[serde(flatten)]
    params: TParams,
    #[serde(skip_serializing_if = "Option::is_none")]
    _meta: Option<ParamsMeta>,
}
