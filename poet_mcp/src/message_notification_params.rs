use serde::Deserialize;
use serde::Serialize;

use crate::log_level::LogLevel;

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct MessageNotificationParams {
    pub data: String,
    pub level: LogLevel,
}
