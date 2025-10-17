use std::collections::BTreeMap;
use std::sync::Arc;

use crate::build_project::build_project_result_stub::BuildProjectResultStub;
use crate::content_document_source::ContentDocumentSource;
use crate::filesystem::memory::Memory;

#[derive(Clone)]
pub struct BuildProjectResult {
    pub changed_since_last_build: Vec<ContentDocumentSource>,
    pub content_document_sources: Arc<BTreeMap<String, ContentDocumentSource>>,
    pub memory_filesystem: Arc<Memory>,
}

impl From<BuildProjectResultStub> for BuildProjectResult {
    fn from(
        BuildProjectResultStub {
            content_document_sources,
            memory_filesystem,
            ..
        }: BuildProjectResultStub,
    ) -> Self {
        Self {
            changed_since_last_build: vec![],
            content_document_sources,
            memory_filesystem,
        }
    }
}
