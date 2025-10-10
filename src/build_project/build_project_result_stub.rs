use std::collections::BTreeMap;
use std::sync::Arc;

use esbuild_metafile::EsbuildMetaFile;
use rayon::prelude::*;

use crate::build_project::build_project_result::BuildProjectResult;
use crate::filesystem::memory::Memory;
use crate::markdown_document_source::MarkdownDocumentSource;

pub struct BuildProjectResultStub {
    pub esbuild_metafile: Arc<EsbuildMetaFile>,
    pub markdown_document_sources: Arc<BTreeMap<String, MarkdownDocumentSource>>,
    pub memory_filesystem: Arc<Memory>,
}

impl BuildProjectResultStub {
    pub fn changed_compared_to(self, other: BuildProjectResult) -> BuildProjectResult {
        let changed_since_last_build: Vec<MarkdownDocumentSource> = self
            .markdown_document_sources
            .values()
            .par_bridge()
            .filter(|markdown_document_source| {
                for other_markdown_document_source in other.markdown_document_sources.values() {
                    if other_markdown_document_source.reference.basename_path
                        == markdown_document_source.reference.basename_path
                    {
                        return other_markdown_document_source.file_entry.contents_hash
                            != markdown_document_source.file_entry.contents_hash;
                    }
                }

                false
            })
            .map(|markdown_document_source| markdown_document_source.clone())
            .collect();

        BuildProjectResult {
            changed_since_last_build,
            esbuild_metafile: self.esbuild_metafile,
            markdown_document_sources: self.markdown_document_sources,
            memory_filesystem: self.memory_filesystem,
        }
    }
}
