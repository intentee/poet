use std::collections::BTreeMap;
use std::sync::Arc;

use esbuild_metafile::esbuild_metafile::EsbuildMetafile;
use rhai_components::rhai_template_renderer::RhaiTemplateRenderer;

use crate::content_document_basename::ContentDocumentBasename;
use crate::content_document_linker::ContentDocumentLinker;
use crate::content_document_source::ContentDocumentSource;
use crate::generated_file::GeneratedFile;

#[derive(Clone)]
pub struct BuildProjectResult {
    pub changed_since_last_build: Vec<ContentDocumentSource>,
    pub content_document_linker: ContentDocumentLinker,
    pub content_document_sources: Arc<BTreeMap<ContentDocumentBasename, ContentDocumentSource>>,
    pub esbuild_metafile: Arc<EsbuildMetafile>,
    pub generated_files: Arc<Vec<GeneratedFile>>,
    pub rhai_template_renderer: RhaiTemplateRenderer,
}
