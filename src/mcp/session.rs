use crate::mcp::log_level::LogLevel;

#[derive(Clone)]
pub struct Session {
    pub log_level: LogLevel,
    pub session_id: String,
}

impl Session {
    pub fn with_log_level(self, log_level: LogLevel) -> Self {
        Self {
            log_level,
            session_id: self.session_id,
        }
    }
}
