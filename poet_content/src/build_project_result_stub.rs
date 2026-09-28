use std::collections::BTreeMap;
use std::sync::Arc;

use esbuild_metafile::esbuild_metafile::EsbuildMetafile;
use poet_filesystem::memory::Memory;

use crate::build_project_result::BuildProjectResult;
use crate::content_document_basename::ContentDocumentBasename;
use crate::content_document_linker::ContentDocumentLinker;
use crate::content_document_source::ContentDocumentSource;

pub struct BuildProjectResultStub {
    pub content_document_linker: ContentDocumentLinker,
    pub content_document_sources: Arc<BTreeMap<ContentDocumentBasename, ContentDocumentSource>>,
    pub esbuild_metafile: Arc<EsbuildMetafile>,
    pub memory_filesystem: Arc<Memory>,
}

impl BuildProjectResultStub {
    #[must_use]
    pub fn changed_compared_to(self, previous_build: &BuildProjectResult) -> BuildProjectResult {
        let changed_since_last_build = self
            .content_document_sources
            .iter()
            .filter(|(basename, content_document_source)| {
                previous_build
                    .content_document_sources
                    .get(*basename)
                    .is_some_and(|previous_content_document_source| {
                        previous_content_document_source.file_entry.contents_hash
                            != content_document_source.file_entry.contents_hash
                    })
            })
            .map(|(_, content_document_source)| content_document_source.clone())
            .collect();

        BuildProjectResult {
            changed_since_last_build,
            ..BuildProjectResult::from(self)
        }
    }
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
