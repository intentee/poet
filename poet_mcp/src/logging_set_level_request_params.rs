use serde::Deserialize;
use serde::Serialize;

use crate::log_level::LogLevel;
use crate::meta::Meta;

#[derive(Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct LoggingSetLevelRequestParams {
    pub level: LogLevel,
    #[serde(rename = "_meta", skip_serializing_if = "Option::is_none")]
    pub meta: Option<Meta>,
}
