use crate::mcp::jsonrpc::role::Role;

pub enum PendingPromptMessage {
    WithRole { chunk: String, role: Role },
    WithoutRole { chunk: String },
}

impl Default for PendingPromptMessage {
    fn default() -> Self {
        Self::WithoutRole {
            chunk: String::new(),
        }
    }
}
