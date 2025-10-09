use std::sync::Arc;

use anyhow::Result;
use tantivy::Index;
use tantivy::IndexReader;
use tantivy::TantivyDocument;
use tantivy::collector::TopDocs;
use tantivy::query::QueryParser;
use tantivy::schema::Schema;
use tantivy::schema::document::Document as _;

use crate::search_index_fields::SearchIndexFields;

pub struct SearchIndexReader {
    pub fields: Arc<SearchIndexFields>,
    pub index: Index,
    pub index_reader: IndexReader,
    pub schema: Schema,
}

impl SearchIndexReader {
    pub fn query(&self, query_str: &str) -> Result<Vec<String>> {
        let mut query_parser = QueryParser::for_index(
            &self.index,
            vec![self.fields.title, self.fields.header, self.fields.paragraph],
        );

        query_parser.set_field_boost(self.fields.title, 4.0);
        query_parser.set_field_boost(self.fields.description, 3.0);
        query_parser.set_field_boost(self.fields.header, 2.0);

        let query = query_parser.parse_query(query_str)?;

        let searcher = self.index_reader.searcher();
        let results = searcher.search(&query, &TopDocs::with_limit(10))?;

        let mut ret = Vec::new();

        for (_score, doc_address) in results {
            ret.push(
                searcher
                    .doc::<TantivyDocument>(doc_address)?
                    .to_json(&self.schema),
            );
        }

        Ok(ret)
    }
}
