use crate::mcp_error::McpError;
use crate::session::Session;

pub enum RequestSession {
    Absent,
    Established(Session),
}

impl RequestSession {
    pub fn established(self) -> Result<Session, McpError> {
        match self {
            Self::Absent => Err(McpError::MissingSessionHeader),
            Self::Established(session) => Ok(session),
        }
    }

    pub const fn require_absent(&self) -> Result<(), McpError> {
        match self {
            Self::Absent => Ok(()),
            Self::Established(_) => Err(McpError::UnexpectedSessionHeader),
        }
    }
}
