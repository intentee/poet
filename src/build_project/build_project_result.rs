use std::sync::Arc;

use dashmap::DashMap;
use esbuild_metafile::EsbuildMetaFile;

use crate::filesystem::memory::Memory;
use crate::markdown_document_reference::MarkdownDocumentReference;

#[derive(Clone)]
pub struct BuildProjectResult {
    pub esbuild_metafile: Arc<EsbuildMetaFile>,
    pub markdown_document_reference_collection: DashMap<String, MarkdownDocumentReference>,
    pub memory_filesystem: Arc<Memory>,
}
