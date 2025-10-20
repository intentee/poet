use std::collections::BTreeMap;
use std::sync::Arc;

use esbuild_metafile::EsbuildMetaFile;
use rayon::prelude::*;

use crate::build_project::build_project_result::BuildProjectResult;
use crate::content_document_basename::ContentDocumentBasename;
use crate::content_document_linker::ContentDocumentLinker;
use crate::content_document_source::ContentDocumentSource;
use crate::filesystem::memory::Memory;

pub struct BuildProjectResultStub {
    pub content_document_linker: ContentDocumentLinker,
    pub content_document_sources: Arc<BTreeMap<ContentDocumentBasename, ContentDocumentSource>>,
    pub esbuild_metafile: Arc<EsbuildMetaFile>,
    pub memory_filesystem: Arc<Memory>,
}

impl BuildProjectResultStub {
    pub fn changed_compared_to(self, other: BuildProjectResult) -> BuildProjectResult {
        let changed_since_last_build: Vec<ContentDocumentSource> = self
            .content_document_sources
            .values()
            .par_bridge()
            .filter(|content_document_source| {
                for other_content_document_source in other.content_document_sources.values() {
                    if other_content_document_source.reference.basename_path
                        == content_document_source.reference.basename_path
                    {
                        return other_content_document_source.file_entry.contents_hash
                            != content_document_source.file_entry.contents_hash;
                    }
                }

                false
            })
            .map(|content_document_source| content_document_source.clone())
            .collect();

        BuildProjectResult {
            changed_since_last_build,
            content_document_linker: self.content_document_linker,
            content_document_sources: self.content_document_sources,
            memory_filesystem: self.memory_filesystem,
        }
    }
}
