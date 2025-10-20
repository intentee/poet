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

use crate::anyhow_error_aggregate::AnyhowErrorAggregate;
use crate::content_document_basename::ContentDocumentBasename;
use crate::content_document_source::ContentDocumentSource;
use crate::mdast_to_tantivy_document::mdast_to_tantivy_document;
use crate::search_index_fields::SearchIndexFields;
use crate::search_index_reader::SearchIndexReader;
use crate::search_index_schema::SearchIndexSchema;

pub struct SearchIndex {
    content_document_sources: Arc<BTreeMap<ContentDocumentBasename, ContentDocumentSource>>,
    fields: Arc<SearchIndexFields>,
    index: Index,
}

impl SearchIndex {
    pub fn create_in_memory(
        content_document_sources: Arc<BTreeMap<ContentDocumentBasename, ContentDocumentSource>>,
    ) -> Self {
        let SearchIndexSchema { fields, schema } = SearchIndexSchema::new();

        let index = Index::create_in_ram(schema.clone());

        Self {
            fields: Arc::new(fields),
            index,
            content_document_sources,
        }
    }

    pub fn index(self) -> Result<SearchIndexReader> {
        let error_collection: AnyhowErrorAggregate = Default::default();
        let fields = self.fields.clone();
        let index_writer: Arc<RwLock<IndexWriter>> =
            Arc::new(RwLock::new(self.index.writer(50_000_000)?));

        self.content_document_sources.par_iter().for_each(
            |(
                _key,
                ContentDocumentSource {
                    mdast, reference, ..
                },
            )| {
                let basename_string: String = reference.basename().to_string();
                let mut document = mdast_to_tantivy_document(fields.clone(), mdast);

                document.add_field_value(fields.basename, &basename_string);
                document.add_field_value(fields.title, &reference.front_matter.title);
                document.add_field_value(fields.description, &reference.front_matter.description);

                if let Err(err) = index_writer
                    .read()
                    .expect("Search index read lock is poisoned")
                    .add_document(document)
                {
                    error_collection.errors.insert(basename_string, err.into());
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

        let index_reader: IndexReader = self
            .index
            .reader_builder()
            .reload_policy(ReloadPolicy::Manual)
            .try_into()?;

        Ok(SearchIndexReader {
            content_document_sources: self.content_document_sources,
            fields: self.fields,
            index: self.index,
            index_reader,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::asset_path_renderer::AssetPathRenderer;
    use crate::build_project::build_project;
    use crate::build_project::build_project_params::BuildProjectParams;
    use crate::build_project::build_project_result_stub::BuildProjectResultStub;
    use crate::compile_shortcodes::compile_shortcodes;
    use crate::filesystem::storage::Storage;
    use crate::search_index_query_params::SearchIndexQueryParams;

    async fn do_build_project() -> Result<BuildProjectResultStub> {
        let public_path: String = "https://example.com".to_string();
        let source_filesystem = Arc::new(Storage {
            base_directory: env!("CARGO_MANIFEST_DIR").into(),
        });
        let rhai_template_renderer = compile_shortcodes(source_filesystem.clone()).await?;

        build_project(BuildProjectParams {
            asset_path_renderer: AssetPathRenderer {
                base_path: public_path.clone(),
            },
            generated_page_base_path: public_path,
            is_watching: false,
            rhai_template_renderer,
            source_filesystem,
        })
        .await
    }

    #[tokio::test]
    async fn test_index_is_searchable() -> Result<()> {
        let BuildProjectResultStub {
            content_document_sources,
            ..
        } = do_build_project().await?;
        let search_index = SearchIndex::create_in_memory(content_document_sources);
        let search_index_reader: SearchIndexReader = search_index.index()?;

        let results = search_index_reader.query(SearchIndexQueryParams {
            cursor: Default::default(),
            query: "test".to_string(),
        })?;

        for result in results {
            println!("{:#?}", result);
        }

        // assert!(false);

        Ok(())
    }
}
