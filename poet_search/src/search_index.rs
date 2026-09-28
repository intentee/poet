use std::collections::BTreeMap;
use std::sync::Arc;

use poet_content::content_document_basename::ContentDocumentBasename;
use poet_content::content_document_source::ContentDocumentSource;
use rayon::iter::IntoParallelRefIterator as _;
use rayon::iter::ParallelIterator as _;
use tantivy::Index;
use tantivy::IndexWriter;
use tantivy::ReloadPolicy;
use tantivy::indexer::IndexWriterOptions;
use tantivy::indexer::UserOperation;

use crate::search_error::SearchError;
use crate::search_index_fields::SearchIndexFields;
use crate::search_index_reader::SearchIndexReader;
use crate::search_index_schema::SearchIndexSchema;

pub struct SearchIndex {
    content_document_sources: Arc<BTreeMap<ContentDocumentBasename, ContentDocumentSource>>,
    fields: SearchIndexFields,
    index: Index,
}

impl SearchIndex {
    #[must_use]
    pub fn create_in_memory(
        content_document_sources: Arc<BTreeMap<ContentDocumentBasename, ContentDocumentSource>>,
    ) -> Self {
        let SearchIndexSchema { fields, schema } = SearchIndexSchema::default();

        Self {
            content_document_sources,
            fields,
            index: Index::create_in_ram(schema),
        }
    }

    pub fn index(self) -> Result<SearchIndexReader, SearchError> {
        let add_operations: Vec<UserOperation> = self
            .content_document_sources
            .par_iter()
            .map(|(_basename, content_document_source)| {
                UserOperation::Add(self.fields.tantivy_document(content_document_source))
            })
            .collect();

        self.index
            .writer_with_options(IndexWriterOptions::builder().build())
            .map_err(SearchError::CreateIndexWriter)
            .and_then(|mut index_writer: IndexWriter| {
                index_writer
                    .run(add_operations)
                    .map_err(SearchError::AddDocuments)
                    .and_then(|_batch_opstamp| {
                        index_writer.commit().map_err(SearchError::CommitIndex)
                    })
            })
            .and_then(|_commit_opstamp| {
                self.index
                    .reader_builder()
                    .reload_policy(ReloadPolicy::Manual)
                    .try_into()
                    .map_err(SearchError::CreateIndexReader)
            })
            .map(|index_reader| SearchIndexReader {
                content_document_sources: self.content_document_sources,
                fields: self.fields,
                index_reader,
                query_parser: self.fields.query_parser(&self.index),
            })
    }
}
