use std::collections::BTreeMap;
use std::sync::Arc;

use esbuild_metafile::EsbuildMetaFile;

use crate::build_project::build_project_result_stub::BuildProjectResultStub;
use crate::filesystem::memory::Memory;
use crate::markdown_document_source::MarkdownDocumentSource;

#[derive(Clone)]
pub struct BuildProjectResult {
    pub changed_since_last_build: Vec<MarkdownDocumentSource>,
    pub esbuild_metafile: Arc<EsbuildMetaFile>,
    pub markdown_document_sources: Arc<BTreeMap<String, MarkdownDocumentSource>>,
    pub memory_filesystem: Arc<Memory>,
}

impl From<BuildProjectResultStub> for BuildProjectResult {
    fn from(
        BuildProjectResultStub {
            esbuild_metafile,
            markdown_document_sources,
            memory_filesystem,
        }: BuildProjectResultStub,
    ) -> Self {
        Self {
            changed_since_last_build: vec![],
            esbuild_metafile,
            markdown_document_sources,
            memory_filesystem,
        }
    }
}
