use std::mem::take;

use poet_mcp::prompt_message::PromptMessage;
use poet_mcp::role::Role;

use crate::pending_prompt_message::PendingPromptMessage;
use crate::prompt_error::PromptError;

#[derive(Default)]
pub struct PromptMessageAccumulator {
    pub messages: Vec<PromptMessage>,
    pending: PendingPromptMessage,
}

impl PromptMessageAccumulator {
    pub fn append(&mut self, appended_chunk: &str) {
        match &mut self.pending {
            PendingPromptMessage::WithRole { chunk, .. }
            | PendingPromptMessage::WithoutRole { chunk } => chunk.push_str(appended_chunk),
        }
    }

    pub fn flush(&mut self) -> Result<(), PromptError> {
        match take(&mut self.pending) {
            PendingPromptMessage::WithRole { chunk, role } => {
                self.messages.push(PromptMessage {
                    content: chunk.into(),
                    role,
                });

                Ok(())
            }
            PendingPromptMessage::WithoutRole { chunk } if chunk.is_empty() => Ok(()),
            PendingPromptMessage::WithoutRole { chunk } => {
                Err(PromptError::MessageWithoutRole { content: chunk })
            }
        }
    }

    pub fn switch_role_to(&mut self, role: Role) -> Result<(), PromptError> {
        self.flush().map(|()| {
            self.pending = PendingPromptMessage::WithRole {
                chunk: String::new(),
                role,
            };
        })
    }
}
