use std::collections::BTreeMap;
use std::sync::Arc;

use anyhow::Result;
use anyhow::anyhow;
use tantivy::Index;
use tantivy::IndexReader;
use tantivy::TantivyDocument;
use tantivy::collector::TopDocs;
use tantivy::query::QueryParser;
use tantivy::schema::Value as _;

use crate::markdown_document_source::MarkdownDocumentSource;
use crate::mcp::list_resources_cursor::ListResourcesCursor;
use crate::search_index_fields::SearchIndexFields;
use crate::search_index_found_document::SearchIndexFoundDocument;
use crate::search_index_query_params::SearchIndexQueryParams;

pub struct SearchIndexReader {
    pub fields: Arc<SearchIndexFields>,
    pub index: Index,
    pub index_reader: IndexReader,
    pub markdown_document_sources: Arc<BTreeMap<String, MarkdownDocumentSource>>,
}

impl SearchIndexReader {
    pub fn query(
        &self,
        SearchIndexQueryParams {
            cursor: ListResourcesCursor { offset, per_page },
            query,
        }: SearchIndexQueryParams,
    ) -> Result<Vec<SearchIndexFoundDocument>> {
        let mut query_parser = QueryParser::for_index(
            &self.index,
            vec![self.fields.title, self.fields.header, self.fields.paragraph],
        );

        query_parser.set_field_boost(self.fields.title, 4.0);
        query_parser.set_field_boost(self.fields.description, 3.0);
        query_parser.set_field_boost(self.fields.header, 2.0);

        let query = query_parser.parse_query(&query)?;

        let searcher = self.index_reader.searcher();
        let results = searcher.search(&query, &TopDocs::with_limit(per_page).and_offset(offset))?;

        let mut ret = Vec::new();

        for (_score, doc_address) in results {
            let tantivy_document: TantivyDocument = searcher.doc::<TantivyDocument>(doc_address)?;

            let basename: &str = tantivy_document
                .get_first(self.fields.basename)
                .ok_or_else(|| anyhow!("Document does not have a stored basename"))?
                .as_str()
                .ok_or_else(|| anyhow!("Unable to convert Tantivy Value to string slice"))?;

            let MarkdownDocumentSource { reference, .. }: &MarkdownDocumentSource = self
                .markdown_document_sources
                .get(basename)
                .ok_or_else(|| anyhow!("There is no document with basename: '{basename}'"))?;

            ret.push(SearchIndexFoundDocument {
                markdown_document_reference: reference.clone(),
            });
        }

        Ok(ret)
    }
}
