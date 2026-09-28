use std::collections::BTreeMap;
use std::sync::Arc;
use std::sync::RwLock;

use anyhow::Result;
use anyhow::anyhow;
use rayon::iter::IntoParallelRefIterator as _;
use rayon::iter::ParallelIterator as _;
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
        let SearchIndexSchema { fields, schema } = SearchIndexSchema::default();

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
    use std::path::Path;

    use poet_filesystem::filesystem::Filesystem as _;
    use poet_filesystem::storage::Storage;
    use tempfile::tempdir;

    use super::*;
    use crate::asset_path_renderer::AssetPathRenderer;
    use crate::build_authors::build_authors;
    use crate::build_project::build_project;
    use crate::build_project::build_project_params::BuildProjectParams;
    use crate::build_project::build_project_result_stub::BuildProjectResultStub;
    use crate::compile_shortcodes::compile_shortcodes;
    use crate::search_index_query_params::SearchIndexQueryParams;

    async fn search_index_reader_for_guide(
        front_matter_description: &str,
    ) -> Result<SearchIndexReader> {
        let directory = tempdir()?;
        let source_filesystem = Arc::new(Storage {
            base_directory: directory.path().to_path_buf(),
        });

        source_filesystem
            .set_file_contents(
                Path::new("shortcodes/Layout.rhai"),
                "fn template(context, props, content) { component { <html>{content}</html> } }",
            )
            .await?;
        source_filesystem
            .set_file_contents(
                Path::new("content/guide.md"),
                &format!(
                    "+++\ndescription = \"{front_matter_description}\"\nlayout = \"Layout\"\ntitle = \"Searchable Guide\"\n+++\n\nUnique body keyword zebra.\n"
                ),
            )
            .await?;

        let rhai_template_renderer = compile_shortcodes(source_filesystem.clone()).await?;
        let authors = build_authors(source_filesystem.clone()).await?;

        let BuildProjectResultStub {
            content_document_sources,
            ..
        } = build_project(BuildProjectParams {
            asset_path_renderer: AssetPathRenderer {
                base_path: "/".to_string(),
            },
            authors,
            esbuild_metafile: Default::default(),
            generated_page_base_path: "/".to_string(),
            generate_sitemap: false,
            is_watching: false,
            rhai_template_renderer,
            source_filesystem,
        })
        .await?;

        SearchIndex::create_in_memory(content_document_sources).index()
    }

    fn found_titles(search_index_reader: &SearchIndexReader, query: &str) -> Result<Vec<String>> {
        Ok(search_index_reader
            .query(SearchIndexQueryParams {
                offset: 0,
                per_page: search_index_reader.content_document_sources.len(),
                query: query.to_string(),
            })?
            .into_iter()
            .map(|found_document| found_document.content_document_reference.front_matter.title)
            .collect())
    }

    #[tokio::test]
    async fn indexes_documents_and_finds_them_by_body_keyword() -> Result<()> {
        let search_index_reader = search_index_reader_for_guide("Guide description").await?;

        assert_eq!(
            found_titles(&search_index_reader, "zebra")?,
            vec!["Searchable Guide".to_string()]
        );

        Ok(())
    }

    #[tokio::test]
    async fn finds_documents_by_description_keyword() -> Result<()> {
        let search_index_reader = search_index_reader_for_guide("Describes the okapi").await?;

        assert_eq!(
            found_titles(&search_index_reader, "okapi")?,
            vec!["Searchable Guide".to_string()]
        );

        Ok(())
    }
}
