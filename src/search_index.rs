use std::collections::BTreeMap;
use std::sync::Arc;
use std::sync::RwLock;

use anyhow::Result;
use anyhow::anyhow;
use rayon::prelude::*;
use tantivy::Index;
use tantivy::IndexReader;
use tantivy::IndexWriter;
use tantivy::ReloadPolicy;
use tantivy::schema::Schema;

use crate::anyhow_error_aggregate::AnyhowErrorAggregate;
use crate::markdown_document_source::MarkdownDocumentSource;
use crate::mdast_to_tantivy_document::mdast_to_tantivy_document;
use crate::search_index_fields::SearchIndexFields;
use crate::search_index_schema::SearchIndexSchema;
use crate::search_index_reader::SearchIndexReader;

pub struct SearchIndex {
    fields: Arc<SearchIndexFields>,
    index: Index,
    schema: Schema,
}

impl SearchIndex {
    pub fn create_in_ram() -> Self {
        let SearchIndexSchema { fields, schema } = SearchIndexSchema::new();

        let index = Index::create_in_ram(schema.clone());

        Self {
            fields: Arc::new(fields),
            index,
            schema,
        }
    }

    pub fn index_markdown_document_sources(
        &self,
        markdown_document_sources: Arc<BTreeMap<String, MarkdownDocumentSource>>,
    ) -> Result<()> {
        let error_collection: AnyhowErrorAggregate = Default::default();
        let fields = self.fields.clone();
        let index_writer: Arc<RwLock<IndexWriter>> =
            Arc::new(RwLock::new(self.index.writer(50_000_000)?));

        markdown_document_sources.par_iter().for_each(
            |(
                _key,
                MarkdownDocumentSource {
                    mdast, reference, ..
                },
            )| {
                let mut document = mdast_to_tantivy_document(fields.clone(), mdast);

                document.add_field_value(fields.title, &reference.front_matter.title);
                document.add_field_value(fields.description, &reference.front_matter.description);

                if let Err(err) = index_writer
                    .read()
                    .expect("Search index read lock is poisoned")
                    .add_document(document)
                {
                    error_collection
                        .errors
                        .insert(reference.basename(), err.into());
                }
            },
        );

        if !error_collection.errors.is_empty() {
            return Err(anyhow!("{error_collection}"));
        }

        index_writer
            .write()
            .expect("Search index write lock is poisoned")
            .commit()?;

        Ok(())
    }

    pub fn reader(self) -> Result<SearchIndexReader> {
        let index_reader: IndexReader = self
            .index
            .reader_builder()
            .reload_policy(ReloadPolicy::Manual)
            .try_into()?;

        Ok(SearchIndexReader {
            fields: self.fields,
            index: self.index,
            index_reader,
            schema: self.schema,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::asset_path_renderer::AssetPathRenderer;
    use crate::build_project::build_project;
    use crate::build_project::build_project_result::BuildProjectResult;
    use crate::compile_shortcodes::compile_shortcodes;
    use crate::filesystem::storage::Storage;

    async fn do_build_project() -> Result<BuildProjectResult> {
        let public_path: String = "https://example.com".to_string();
        let source_filesystem = Arc::new(Storage {
            base_directory: env!("CARGO_MANIFEST_DIR").into(),
        });
        let rhai_template_renderer = compile_shortcodes(source_filesystem.clone()).await?;

        build_project(
            AssetPathRenderer {
                base_path: public_path.clone(),
            },
            public_path,
            false,
            rhai_template_renderer,
            source_filesystem,
        )
        .await
    }

    #[tokio::test]
    async fn test_index_is_searchable() -> Result<()> {
        let BuildProjectResult {
            markdown_document_sources,
            ..
        } = do_build_project().await?;
        let search_index = SearchIndex::create_in_ram();

        search_index.index_markdown_document_sources(markdown_document_sources)?;

        let search_index_reader = search_index.reader()?;
        let results = search_index_reader.query("test")?;

        for result in results {
            println!("{:#?}", result);
        }

        assert!(false);

        Ok(())
    }
}
