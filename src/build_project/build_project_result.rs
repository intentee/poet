use std::collections::BTreeMap;
use std::sync::Arc;

use esbuild_metafile::EsbuildMetaFile;
use rayon::prelude::*;

use crate::filesystem::memory::Memory;
use crate::markdown_document_reference::MarkdownDocumentReference;
use crate::markdown_document_source::MarkdownDocumentSource;

#[derive(Clone)]
pub struct BuildProjectResult {
    pub esbuild_metafile: Arc<EsbuildMetaFile>,
    pub markdown_document_sources: Arc<BTreeMap<String, MarkdownDocumentSource>>,
    pub memory_filesystem: Arc<Memory>,
}

impl BuildProjectResult {
    pub fn changed_compared_to(&self, other: &Self) -> Vec<MarkdownDocumentReference> {
        self.markdown_document_sources
            .values()
            .into_iter()
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
            .map(|markdown_document_source| markdown_document_source.reference.clone())
            .collect()
    }
}
