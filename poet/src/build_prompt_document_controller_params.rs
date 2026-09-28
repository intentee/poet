use std::sync::Arc;

use esbuild_metafile::esbuild_metafile::EsbuildMetafile;
use poet_filesystem::file_entry::FileEntry;
use rhai_components::rhai_template_renderer::RhaiTemplateRenderer;

use crate::asset_path_renderer::AssetPathRenderer;
use crate::content_document_linker::ContentDocumentLinker;

pub struct BuildPromptDocumentControllerParams {
    pub asset_path_renderer: AssetPathRenderer,
    pub content_document_linker: ContentDocumentLinker,
    pub esbuild_metafile: Arc<EsbuildMetafile>,
    pub file: FileEntry,
    pub name: String,
    pub rhai_template_renderer: RhaiTemplateRenderer,
}
