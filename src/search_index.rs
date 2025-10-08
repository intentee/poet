use std::collections::BTreeMap;
use std::sync::Arc;
use std::sync::RwLock;

use anyhow::Result;
use rayon::prelude::*;
use tantivy::Index;
use tantivy::IndexWriter;
use tantivy::schema::Schema;
use tantivy::schema::TEXT;

use crate::markdown_document_source::MarkdownDocumentSource;
use crate::mdast_to_tantivy_document::mdast_to_tantivy_document;
use crate::search_index_fields::SearchIndexFields;

pub struct SearchIndex {
    fields: Arc<SearchIndexFields>,
    index: Index,
}

impl SearchIndex {
    pub fn new() -> Self {
        let mut schema_builder = Schema::builder();

        let title = schema_builder.add_text_field("title", TEXT);
        let description = schema_builder.add_text_field("description", TEXT);

        let schema = schema_builder.build();
        let index = Index::create_in_ram(schema);

        Self {
            fields: Arc::new(SearchIndexFields { description, title }),
            index,
        }
    }

    pub fn index_markdown_document_sources(
        &self,
        markdown_document_sources: Arc<BTreeMap<String, MarkdownDocumentSource>>,
    ) -> Result<()> {
        let fields = self.fields.clone();
        let index_writer: Arc<RwLock<IndexWriter>> =
            Arc::new(RwLock::new(self.index.writer(100_000_000)?));

        markdown_document_sources.par_iter().for_each(
            |(_key, MarkdownDocumentSource { mdast, .. })| {
                index_writer
                    .read()
                    .expect("Read index lock is poisoned")
                    .add_document(mdast_to_tantivy_document(fields.clone(), mdast));
            },
        );

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_index_is_searchable() -> Result<()> {
        Ok(())
    }
}
