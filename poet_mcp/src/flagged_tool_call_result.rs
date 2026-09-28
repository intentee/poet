use serde::Deserialize;
use serde::Serialize;

#[derive(Deserialize, Serialize)]
pub struct FlaggedToolCallResult<TResult> {
    #[serde(flatten)]
    pub result: TResult,
    #[serde(rename = "isError")]
    pub is_error: bool,
}
