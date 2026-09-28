use std::collections::BTreeMap;
use std::mem::take;
use std::sync::Arc;
use std::sync::Mutex;
use std::sync::MutexGuard;

use poet_assets::asset_manager::AssetManager;
use poet_content::content_document_linker::ContentDocumentLinker;
use poet_mcp::prompt_message::PromptMessage;
use poet_mcp::role::Role;
use rhai::CustomType;
use rhai::Dynamic;
use rhai::EvalAltResult;
use rhai::Map;
use rhai::TypeBuilder;

use crate::prompt_document_argument_with_input::PromptDocumentArgumentWithInput;
use crate::prompt_document_front_matter::PromptDocumentFrontMatter;
use crate::prompt_error::PromptError;
use crate::prompt_message_accumulator::PromptMessageAccumulator;

#[derive(Clone)]
pub struct PromptDocumentComponentContext {
    pub arguments: BTreeMap<String, PromptDocumentArgumentWithInput>,
    pub asset_manager: AssetManager,
    pub content_document_linker: ContentDocumentLinker,
    pub front_matter: PromptDocumentFrontMatter,
    pub prompt_message_accumulator: Arc<Mutex<PromptMessageAccumulator>>,
}

impl PromptDocumentComponentContext {
    pub fn append_to_message(&self, chunk: &str) {
        self.lock_prompt_message_accumulator().append(chunk);
    }

    pub fn flush(&self) -> Result<(), PromptError> {
        self.lock_prompt_message_accumulator().flush()
    }

    pub fn switch_role_to(&self, role_name: &str) -> Result<(), PromptError> {
        role_name
            .parse::<Role>()
            .map_err(|source| PromptError::UnknownRole {
                role_name: role_name.to_owned(),
                source,
            })
            .and_then(|role| self.lock_prompt_message_accumulator().switch_role_to(role))
    }

    #[must_use]
    pub fn take_prompt_messages(&self) -> Vec<PromptMessage> {
        take(&mut self.lock_prompt_message_accumulator().messages)
    }

    #[expect(
        clippy::expect_used,
        reason = "a poisoned lock means another prompt evaluation panicked while holding it"
    )]
    fn lock_prompt_message_accumulator(&self) -> MutexGuard<'_, PromptMessageAccumulator> {
        self.prompt_message_accumulator
            .lock()
            .expect("Prompt message accumulator lock is poisoned")
    }

    fn rhai_append_to_message(&mut self, chunk: &str) {
        self.append_to_message(chunk);
    }

    fn rhai_get_arguments(&mut self) -> Map {
        self.arguments
            .iter()
            .map(|(name, argument)| (name.into(), Dynamic::from(argument.clone())))
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

    fn rhai_switch_role_to(&mut self, role_name: &str) -> Result<(), Box<EvalAltResult>> {
        Ok(self.switch_role_to(role_name)?)
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
