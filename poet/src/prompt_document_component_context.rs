use std::collections::HashMap;
use std::mem::take;
use std::sync::Arc;
use std::sync::Mutex;
use std::sync::MutexGuard;

use anyhow::Result;
use poet_mcp::prompt_message::PromptMessage;
use poet_mcp::role::Role;
use rhai::CustomType;
use rhai::Dynamic;
use rhai::EvalAltResult;
use rhai::Map;
use rhai::TypeBuilder;

use crate::asset_manager::AssetManager;
use crate::content_document_linker::ContentDocumentLinker;
use crate::prompt_document_front_matter::PromptDocumentFrontMatter;
use crate::prompt_document_front_matter::argument_with_input::ArgumentWithInput;
use crate::prompt_message_accumulator::PromptMessageAccumulator;

#[derive(Clone)]
pub struct PromptDocumentComponentContext {
    pub arguments: HashMap<String, ArgumentWithInput>,
    pub asset_manager: AssetManager,
    pub content_document_linker: ContentDocumentLinker,
    pub front_matter: PromptDocumentFrontMatter,
    pub prompt_message_accumulator: Arc<Mutex<PromptMessageAccumulator>>,
}

impl PromptDocumentComponentContext {
    pub fn append_to_message(&self, chunk: &str) {
        self.lock_prompt_message_accumulator().append(chunk);
    }

    pub fn flush(&self) -> Result<()> {
        self.lock_prompt_message_accumulator().flush()
    }

    pub fn switch_role_to(&self, role: Role) -> Result<()> {
        self.lock_prompt_message_accumulator().switch_role_to(role)
    }

    pub fn take_prompt_messages(&self) -> Vec<PromptMessage> {
        take(&mut self.lock_prompt_message_accumulator().messages)
    }

    fn lock_prompt_message_accumulator(&self) -> MutexGuard<'_, PromptMessageAccumulator> {
        self.prompt_message_accumulator
            .lock()
            .expect("Prompt message accumulator lock is poisoned")
    }

    fn rhai_append_to_message(&mut self, chunk: String) {
        self.append_to_message(&chunk);
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
        let role: Role = match role_string.parse() {
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

#[cfg(test)]
mod tests {
    use esbuild_metafile::esbuild_metafile::EsbuildMetafile;
    use poet_mcp::content_block::ContentBlock;

    use super::*;
    use crate::asset_path_renderer::AssetPathRenderer;
    use crate::content_document_front_matter::ContentDocumentFrontMatter;
    use crate::content_document_reference::ContentDocumentReference;

    fn linker() -> ContentDocumentLinker {
        let mut content_document_by_basename = HashMap::new();

        content_document_by_basename.insert(
            "guide".to_string().into(),
            ContentDocumentReference {
                basename_path: "guide".into(),
                front_matter: ContentDocumentFrontMatter::mock("guide"),
                generated_page_base_path: "/".to_string(),
            },
        );

        ContentDocumentLinker {
            content_document_basename_by_id: Arc::new(HashMap::new()),
            content_document_by_basename: Arc::new(content_document_by_basename),
        }
    }

    fn context() -> PromptDocumentComponentContext {
        PromptDocumentComponentContext {
            arguments: HashMap::new(),
            asset_manager: AssetManager::from_esbuild_metafile(
                Arc::new(EsbuildMetafile::default()),
                AssetPathRenderer {
                    base_path: "/".to_string(),
                },
            ),
            content_document_linker: linker(),
            front_matter: PromptDocumentFrontMatter {
                arguments: HashMap::new(),
                description: "description".to_string(),
                title: "title".to_string(),
            },
            prompt_message_accumulator: Default::default(),
        }
    }

    #[test]
    fn rhai_append_adds_chunk_to_current_message() -> Result<()> {
        let mut context = context();

        context.switch_role_to(Role::User)?;
        context.rhai_append_to_message("piece".to_string());
        context.flush()?;

        assert_eq!(
            context.take_prompt_messages(),
            vec![PromptMessage {
                content: ContentBlock::from("piece"),
                role: Role::User,
            }]
        );

        Ok(())
    }

    #[test]
    fn rhai_link_resolves_internal_path() -> Result<()> {
        let mut context = context();

        assert_eq!(context.rhai_link_to("guide")?, "/guide/");

        Ok(())
    }

    #[test]
    fn rhai_link_fails_for_unknown_path() {
        let mut context = context();

        assert!(context.rhai_link_to("ghost").is_err());
    }

    #[test]
    fn rhai_switch_role_accepts_known_role() -> Result<()> {
        let mut context = context();

        context.rhai_switch_role_to("assistant".to_string())?;
        context.append_to_message("answer");
        context.flush()?;

        assert_eq!(
            context.take_prompt_messages(),
            vec![PromptMessage {
                content: ContentBlock::from("answer"),
                role: Role::Assistant,
            }]
        );

        Ok(())
    }

    #[test]
    fn rhai_switch_role_rejects_unknown_role() {
        let mut context = context();

        assert!(
            context
                .rhai_switch_role_to("moderator".to_string())
                .is_err()
        );
    }

    #[test]
    fn rhai_switch_role_fails_when_orphan_chunk_cannot_flush() -> Result<()> {
        let mut context = context();

        context.append_to_message("orphan");

        assert!(context.rhai_switch_role_to("user".to_string()).is_err());

        Ok(())
    }
}
