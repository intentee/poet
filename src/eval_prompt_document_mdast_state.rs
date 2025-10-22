use anyhow::Result;
use anyhow::anyhow;

use crate::mcp::jsonrpc::role::Role;
use crate::mcp::prompt_message::PromptMessage;

fn trim_chunk(chunk: String) -> Result<String> {
    Ok(chunk
        .trim()
        .strip_prefix(':')
        .ok_or_else(|| anyhow!("Unable to strip chunk prefix"))?
        .trim_start()
        .to_string())
}

#[derive(Debug, Default)]
pub struct EvalPromptDocumentMdastState {
    current_role: Option<Role>,
    pub prompt_messages: Vec<PromptMessage>,
    unprocessed_message_chunk: String,
}

impl EvalPromptDocumentMdastState {
    pub fn append_to_message(&mut self, chunk: String) -> Result<()> {
        if chunk.is_empty() {
            return Ok(());
        }

        let trimmed_chunk = trim_chunk(chunk)?;

        self.unprocessed_message_chunk.push_str(&trimmed_chunk);

        Ok(())
    }

    pub fn flush(&mut self) -> Result<()> {
        if let Some(role) = self.current_role.take() {
            self.prompt_messages.push(PromptMessage {
                content: self.unprocessed_message_chunk.clone().into(),
                role,
            });

            self.unprocessed_message_chunk = "".to_string();

            Ok(())
        } else if self.unprocessed_message_chunk.is_empty() {
            Ok(())
        } else {
            Err(anyhow!("Tried to flush messages, but there is no role set"))
        }
    }

    pub fn switch_role_to(&mut self, role: Role) -> Result<()> {
        self.flush()?;
        self.current_role = Some(role);

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_chunk_trim() -> Result<()> {
        assert_eq!(
            trim_chunk(
                r#"
                : foo bar
            "#
                .to_string()
            )?,
            "foo bar".to_string(),
        );

        Ok(())
    }
}
