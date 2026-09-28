use std::collections::BTreeMap;
use std::sync::Arc;

use esbuild_metafile::esbuild_metafile::EsbuildMetafile;
use poet_assets::asset_manager::AssetManager;
use poet_assets::asset_path_renderer::AssetPathRenderer;
use poet_content::content_document_linker::ContentDocumentLinker;
use rhai_components::rhai_template_renderer::RhaiTemplateRenderer;

use crate::prompt_document_argument_with_input::PromptDocumentArgumentWithInput;
use crate::prompt_document_component_context::PromptDocumentComponentContext;
use crate::prompt_document_front_matter::PromptDocumentFrontMatter;

pub struct PromptRenderingContext {
    pub asset_path_renderer: AssetPathRenderer,
    pub content_document_linker: ContentDocumentLinker,
    pub esbuild_metafile: Arc<EsbuildMetafile>,
    pub rhai_template_renderer: RhaiTemplateRenderer,
}

impl PromptRenderingContext {
    #[must_use]
    pub fn component_context(
        &self,
        arguments: BTreeMap<String, PromptDocumentArgumentWithInput>,
        front_matter: PromptDocumentFrontMatter,
    ) -> PromptDocumentComponentContext {
        PromptDocumentComponentContext {
            arguments,
            asset_manager: AssetManager::from_esbuild_metafile(
                self.esbuild_metafile.clone(),
                self.asset_path_renderer.clone(),
            ),
            content_document_linker: self.content_document_linker.clone(),
            front_matter,
            prompt_message_accumulator: Arc::default(),
        }
    }
}
