use std::sync::Arc;

use esbuild_metafile::esbuild_metafile::EsbuildMetafile;
use poet_assets::asset_path_renderer::AssetPathRenderer;
use poet_filesystem::storage::Storage;
use rhai_components::rhai_template_renderer::RhaiTemplateRenderer;

use poet_content::content_document_linker::ContentDocumentLinker;

pub struct BuildPromptControllerCollectionParams {
    pub asset_path_renderer: AssetPathRenderer,
    pub content_document_linker: ContentDocumentLinker,
    pub esbuild_metafile: Arc<EsbuildMetafile>,
    pub rhai_template_renderer: RhaiTemplateRenderer,
    pub source_filesystem: Arc<Storage>,
}
