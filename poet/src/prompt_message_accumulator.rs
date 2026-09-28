use std::mem::take;

use anyhow::Result;
use anyhow::anyhow;

use crate::mcp::jsonrpc::role::Role;
use crate::mcp::prompt_message::PromptMessage;
use crate::pending_prompt_message::PendingPromptMessage;

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

    pub fn flush(&mut self) -> Result<()> {
        match take(&mut self.pending) {
            PendingPromptMessage::WithRole { chunk, role } => {
                self.messages.push(PromptMessage {
                    content: chunk.into(),
                    role,
                });

                Ok(())
            }
            PendingPromptMessage::WithoutRole { chunk } if chunk.is_empty() => Ok(()),
            PendingPromptMessage::WithoutRole { .. } => {
                Err(anyhow!("Tried to flush messages, but there is no role set"))
            }
        }
    }

    pub fn switch_role_to(&mut self, role: Role) -> Result<()> {
        self.flush()?;
        self.pending = PendingPromptMessage::WithRole {
            chunk: String::new(),
            role,
        };

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use anyhow::Result;

    use super::PromptMessageAccumulator;
    use crate::mcp::content_block::ContentBlock;
    use crate::mcp::jsonrpc::role::Role;
    use crate::mcp::prompt_message::PromptMessage;

    #[test]
    fn flush_fails_when_chunk_present_without_role() {
        let mut accumulator = PromptMessageAccumulator::default();

        accumulator.append("orphan");

        assert!(accumulator.flush().is_err());
    }

    #[test]
    fn flush_without_role_or_chunk_adds_no_message() -> Result<()> {
        let mut accumulator = PromptMessageAccumulator::default();

        accumulator.flush()?;

        assert!(accumulator.messages.is_empty());

        Ok(())
    }

    #[test]
    fn switch_role_flushes_previous_message() -> Result<()> {
        let mut accumulator = PromptMessageAccumulator::default();

        accumulator.switch_role_to(Role::User)?;
        accumulator.append("hello");
        accumulator.switch_role_to(Role::Assistant)?;

        assert_eq!(
            accumulator.messages,
            vec![PromptMessage {
                content: ContentBlock::from("hello"),
                role: Role::User,
            }]
        );

        Ok(())
    }
}
