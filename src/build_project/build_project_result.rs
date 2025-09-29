use std::collections::BTreeMap;
use std::sync::Arc;

use esbuild_metafile::EsbuildMetaFile;

use crate::filesystem::memory::Memory;
use crate::markdown_document_reference::MarkdownDocumentReference;
use crate::markdown_document_source::MarkdownDocumentSource;

#[derive(Clone)]
pub struct BuildProjectResult {
    pub esbuild_metafile: Arc<EsbuildMetaFile>,
    pub markdown_document_reference_collection: Arc<BTreeMap<String, MarkdownDocumentReference>>,
    pub markdown_document_sources: Arc<BTreeMap<String, MarkdownDocumentSource>>,
    pub memory_filesystem: Arc<Memory>,
}
