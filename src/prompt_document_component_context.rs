use std::collections::HashMap;

use anyhow::Result;
use anyhow::anyhow;
use rhai::CustomType;
use rhai::Dynamic;
use rhai::EvalAltResult;
use rhai::Map;
use rhai::TypeBuilder;

use crate::asset_manager::AssetManager;
use crate::content_document_linker::ContentDocumentLinker;
use crate::mcp::jsonrpc::role::Role;
use crate::mcp::prompt_message::PromptMessage;
use crate::prompt_document_front_matter::PromptDocumentFrontMatter;
use crate::prompt_document_front_matter::argument_with_input::ArgumentWithInput;

fn trim_chunk(chunk: String) -> Result<String> {
    Ok(chunk
        .trim()
        .strip_prefix(':')
        .ok_or_else(|| anyhow!("Unable to strip chunk prefix"))?
        .trim_start()
        .to_string())
}

#[derive(Clone)]
pub struct PromptDocumentComponentContext {
    pub arguments: HashMap<String, ArgumentWithInput>,
    pub asset_manager: AssetManager,
    pub content_document_linker: ContentDocumentLinker,
    pub current_role: Option<Role>,
    pub front_matter: PromptDocumentFrontMatter,
    pub prompt_messages: Vec<PromptMessage>,
    pub unprocessed_message_chunk: String,
}

impl PromptDocumentComponentContext {
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

    fn rhai_get_arguments(&mut self) -> Map {
        self.arguments
            .clone()
            .into_iter()
            .map(|(name, argument)| (name.into(), Dynamic::from(argument)))
            .collect()
    }

    fn rhai_get_assets(&mut self) -> AssetManager {
        self.asset_manager.clone()
    }

    fn rhai_get_front_matter(&mut self) -> PromptDocumentFrontMatter {
        self.front_matter.clone()
    }

    fn rhai_link_to(&mut self, path: &str) -> Result<String, Box<EvalAltResult>> {
        Ok(self.content_document_linker.link_to(path)?)
    }
}

impl CustomType for PromptDocumentComponentContext {
    fn build(mut builder: TypeBuilder<Self>) {
        builder
            .with_name("PromptDocumentComponentContext")
            .with_get("arguments", Self::rhai_get_arguments)
            .with_get("assets", Self::rhai_get_assets)
            .with_get("front_matter", Self::rhai_get_front_matter)
            .with_fn("link_to", Self::rhai_link_to);
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
