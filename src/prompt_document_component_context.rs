use std::collections::HashMap;

use rhai::CustomType;
use rhai::Dynamic;
use rhai::EvalAltResult;
use rhai::Map;
use rhai::TypeBuilder;

use crate::asset_manager::AssetManager;
use crate::content_document_linker::ContentDocumentLinker;
use crate::prompt_document_front_matter::PromptDocumentFrontMatter;
use crate::prompt_document_front_matter::argument_with_input::ArgumentWithInput;

#[derive(Clone)]
pub struct PromptDocumentComponentContext {
    pub arguments: HashMap<String, ArgumentWithInput>,
    pub asset_manager: AssetManager,
    pub content_document_linker: ContentDocumentLinker,
    pub front_matter: PromptDocumentFrontMatter,
}

impl PromptDocumentComponentContext {
    pub fn rhai_get_arguments(&mut self) -> Map {
        self.arguments
            .clone()
            .into_iter()
            .map(|(name, argument)| (name.into(), Dynamic::from(argument)))
            .collect()
    }

    pub fn rhai_get_assets(&mut self) -> AssetManager {
        self.asset_manager.clone()
    }

    pub fn rhai_get_front_matter(&mut self) -> PromptDocumentFrontMatter {
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
