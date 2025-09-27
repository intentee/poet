use std::collections::BTreeMap;
use std::sync::Arc;

use esbuild_metafile::EsbuildMetaFile;

use crate::filesystem::memory::Memory;
use crate::markdown_document_reference::MarkdownDocumentReference;

#[derive(Clone)]
pub struct BuildProjectResult {
    pub esbuild_metafile: Arc<EsbuildMetaFile>,
    pub markdown_document_reference_collection: Arc<BTreeMap<String, MarkdownDocumentReference>>,
    pub memory_filesystem: Arc<Memory>,
}
