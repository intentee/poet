use std::collections::HashMap;
use std::mem::take;
use std::sync::Arc;
use std::sync::RwLock;

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

#[derive(Clone)]
pub struct PromptDocumentComponentContext {
    pub arguments: HashMap<String, ArgumentWithInput>,
    pub asset_manager: AssetManager,
    pub content_document_linker: ContentDocumentLinker,
    pub current_role: Option<Role>,
    pub front_matter: PromptDocumentFrontMatter,
    pub prompt_messages: Vec<PromptMessage>,
    pub unprocessed_message_chunk: Arc<RwLock<String>>,
}

impl PromptDocumentComponentContext {
    pub fn append_to_message(&mut self, chunk: String) -> Result<()> {
        if !chunk.is_empty() {
            let mut unprocessed_message_chunk = self
                .unprocessed_message_chunk
                .write()
                .expect("Unprocessed message lock is poisoned");

            unprocessed_message_chunk.push_str(&chunk);
        }

        Ok(())
    }

    pub fn flush(&mut self) -> Result<()> {
        let unprocessed_message_chunk = take(
            &mut *self
                .unprocessed_message_chunk
                .write()
                .expect("Unprocessed message lock is poisoned"),
        );

        if let Some(role) = self.current_role.take() {
            self.prompt_messages.push(PromptMessage {
                content: unprocessed_message_chunk.into(),
                role,
            });

            Ok(())
        } else if unprocessed_message_chunk.is_empty() {
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

    fn rhai_append_to_message(&mut self, chunk: String) -> Result<(), Box<EvalAltResult>> {
        if let Err(err) = self.append_to_message(chunk) {
            Err(Box::new(EvalAltResult::ErrorSystem(
                "Unable to append chunk".to_string(),
                err.into(),
            )))
        } else {
            Ok(())
        }
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

    fn rhai_switch_role_to(&mut self, role_string: String) -> Result<(), Box<EvalAltResult>> {
        let role: Role = match role_string.clone().try_into() {
            Ok(role) => role,
            Err(err) => {
                return Err(Box::new(EvalAltResult::ErrorSystem(
                    format!(
                        "Unknown role name: '{role_string} (you can only use 'assistant' or 'user')"
                    ),
                    err.into(),
                )));
            }
        };

        if let Err(err) = self.switch_role_to(role) {
            Err(Box::new(EvalAltResult::ErrorSystem(
                "Unable to switch to role".to_string(),
                err.into(),
            )))
        } else {
            Ok(())
        }
    }
}

impl CustomType for PromptDocumentComponentContext {
    fn build(mut builder: TypeBuilder<Self>) {
        builder
            .with_name("PromptDocumentComponentContext")
            .with_get("arguments", Self::rhai_get_arguments)
            .with_get("assets", Self::rhai_get_assets)
            .with_get("front_matter", Self::rhai_get_front_matter)
            .with_fn("append_to_message", Self::rhai_append_to_message)
            .with_fn("link_to", Self::rhai_link_to)
            .with_fn("switch_role_to", Self::rhai_switch_role_to);
    }
}
