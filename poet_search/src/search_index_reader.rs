use std::collections::BTreeMap;
use std::sync::Arc;

use poet_content::content_document_basename::ContentDocumentBasename;
use poet_content::content_document_source::ContentDocumentSource;
use tantivy::DocAddress;
use tantivy::IndexReader;
use tantivy::Searcher;
use tantivy::TantivyDocument;
use tantivy::collector::TopDocs;
use tantivy::query::QueryParser;
use tantivy::schema::Value as _;

use crate::search_error::SearchError;
use crate::search_index_fields::SearchIndexFields;
use crate::search_index_found_document::SearchIndexFoundDocument;
use crate::search_index_query_params::SearchIndexQueryParams;

pub struct SearchIndexReader {
    pub content_document_sources: Arc<BTreeMap<ContentDocumentBasename, ContentDocumentSource>>,
    pub fields: SearchIndexFields,
    pub index_reader: IndexReader,
    pub query_parser: QueryParser,
}

impl SearchIndexReader {
    pub fn query(
        &self,
        SearchIndexQueryParams {
            offset,
            per_page,
            query,
        }: SearchIndexQueryParams,
    ) -> Result<Vec<SearchIndexFoundDocument>, SearchError> {
        let searcher = self.index_reader.searcher();

        self.query_parser
            .parse_query(&query)
            .map_err(|source| SearchError::ParseQuery { query, source })
            .and_then(|parsed_query| {
                searcher
                    .search(
                        &parsed_query,
                        &TopDocs::with_limit(per_page)
                            .and_offset(offset)
                            .order_by_score(),
                    )
                    .map_err(SearchError::Search)
            })
            .and_then(|scored_document_addresses| {
                scored_document_addresses
                    .into_iter()
                    .map(|(_score, document_address)| {
                        self.found_document(&searcher, document_address)
                    })
                    .collect()
            })
    }

    fn found_document(
        &self,
        searcher: &Searcher,
        document_address: DocAddress,
    ) -> Result<SearchIndexFoundDocument, SearchError> {
        searcher
            .doc::<TantivyDocument>(document_address)
            .map_err(SearchError::ReadFoundDocument)
            .and_then(|tantivy_document| {
                tantivy_document
                    .get_first(self.fields.basename)
                    .ok_or(SearchError::FoundDocumentWithoutBasename { document_address })
                    .and_then(|basename_value| {
                        basename_value
                            .as_str()
                            .map(|basename| ContentDocumentBasename(basename.to_owned()))
                            .ok_or(SearchError::FoundDocumentBasenameNotText { document_address })
                    })
            })
            .and_then(|basename| {
                self.content_document_sources
                    .get(&basename)
                    .map(
                        |ContentDocumentSource { reference, .. }| SearchIndexFoundDocument {
                            content_document_reference: reference.clone(),
                        },
                    )
                    .ok_or(SearchError::FoundDocumentNotIndexed { basename })
            })
    }
}
