use std::collections::BTreeMap;
use std::sync::Arc;

use esbuild_metafile::EsbuildMetaFile;

use crate::build_project::build_project_result_stub::BuildProjectResultStub;
use crate::content_document_basename::ContentDocumentBasename;
use crate::content_document_linker::ContentDocumentLinker;
use crate::content_document_source::ContentDocumentSource;
use crate::filesystem::memory::Memory;

#[derive(Clone)]
pub struct BuildProjectResult {
    pub changed_since_last_build: Vec<ContentDocumentSource>,
    pub content_document_linker: ContentDocumentLinker,
    pub content_document_sources: Arc<BTreeMap<ContentDocumentBasename, ContentDocumentSource>>,
    pub esbuild_metafile: Arc<EsbuildMetaFile>,
    pub memory_filesystem: Arc<Memory>,
}

impl From<BuildProjectResultStub> for BuildProjectResult {
    fn from(
        BuildProjectResultStub {
            content_document_linker,
            content_document_sources,
            esbuild_metafile,
            memory_filesystem,
        }: BuildProjectResultStub,
    ) -> Self {
        Self {
            changed_since_last_build: vec![],
            content_document_linker,
            content_document_sources,
            esbuild_metafile,
            memory_filesystem,
        }
    }
}
