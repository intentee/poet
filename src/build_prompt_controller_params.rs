use std::sync::Arc;

use esbuild_metafile::EsbuildMetaFile;

use crate::asset_path_renderer::AssetPathRenderer;
use crate::content_document_linker::ContentDocumentLinker;
use crate::filesystem::file_entry::FileEntry;
use crate::rhai_template_renderer::RhaiTemplateRenderer;

pub struct BuildPromptControllerParams {
    pub asset_path_renderer: AssetPathRenderer,
    pub content_document_linker: ContentDocumentLinker,
    pub esbuild_metafile: Arc<EsbuildMetaFile>,
    pub file: FileEntry,
    pub rhai_template_renderer: RhaiTemplateRenderer,
}
