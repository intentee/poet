use rhai::CustomType;
use rhai::EvalAltResult;
use rhai::TypeBuilder;

use crate::asset_manager::AssetManager;
use crate::content_document_linker::ContentDocumentLinker;
use crate::prompt_document_front_matter::PromptDocumentFrontMatter;

#[derive(Clone)]
pub struct PromptDocumentComponentContext {
    pub asset_manager: AssetManager,
    pub content_document_linker: ContentDocumentLinker,
    pub front_matter: PromptDocumentFrontMatter,
}

impl PromptDocumentComponentContext {
    pub fn rhai_get_assets(&mut self) -> AssetManager {
        self.asset_manager.clone()
    }

    fn rhai_link_to(&mut self, path: &str) -> Result<String, Box<EvalAltResult>> {
        Ok(self.content_document_linker.link_to(path)?)
    }
}

impl CustomType for PromptDocumentComponentContext {
    fn build(mut builder: TypeBuilder<Self>) {
        builder
            .with_name("PromptDocumentComponentContext")
            .with_get("assets", Self::rhai_get_assets)
            .with_fn("link_to", Self::rhai_link_to);
    }
}
