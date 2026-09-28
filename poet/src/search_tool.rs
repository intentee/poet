use async_trait::async_trait;
use poet_content::content_document_reference::ContentDocumentReference;
use poet_mcp::content_block::ContentBlock;
use poet_mcp::provider_error::ProviderError;
use poet_mcp::resource_link::ResourceLink;
use poet_mcp::resource_provider::ResourceProvider as _;
use poet_mcp::tool_call_error_message::ToolCallErrorMessage;
use poet_mcp::tool_call_result::ToolCallResult;
use poet_mcp::tool_call_success::ToolCallSuccess;
use poet_mcp::tool_provider::ToolProvider;
use poet_mcp::tool_responder::ToolResponder;
use schemars::JsonSchema;
use serde::Deserialize;
use serde::Serialize;
use tokio::task::spawn_blocking;

use crate::holder::Holder as _;
use crate::mcp_resource_provider_content_documents::McpResourceProviderContentDocuments;
use crate::search_index_found_document::SearchIndexFoundDocument;
use crate::search_index_query_params::SearchIndexQueryParams;
use crate::search_index_reader_holder::SearchIndexReaderHolder;

const SEARCH_RESULTS_PER_PAGE: usize = 20;

#[derive(Deserialize, JsonSchema, Serialize)]
pub struct SearchToolProviderInput {
    pub query: String,
}

#[derive(Deserialize, JsonSchema, Serialize)]
pub struct SearchToolProviderOutput {}

pub struct SearchTool {
    pub mcp_resource_provider_content_documents: McpResourceProviderContentDocuments,
    pub search_index_reader_holder: SearchIndexReaderHolder,
}

impl SearchTool {
    fn resource_link(
        &self,
        SearchIndexFoundDocument {
            content_document_reference:
                content_document_reference @ ContentDocumentReference { front_matter, .. },
        }: &SearchIndexFoundDocument,
    ) -> ContentBlock {
        ContentBlock::ResourceLink(ResourceLink {
            description: Some(front_matter.description.clone()),
            mime_type: Some("text/markdown".to_owned()),
            name: front_matter.title.clone(),
            title: Some(front_matter.title.clone()),
            uri: self
                .mcp_resource_provider_content_documents
                .resource_uri(&content_document_reference.basename().to_string()),
        })
    }
}

impl ToolProvider for SearchTool {
    type Input = SearchToolProviderInput;
    type Output = SearchToolProviderOutput;

    fn name(&self) -> String {
        "search".to_owned()
    }
}

#[async_trait]
impl ToolResponder<Self> for SearchTool {
    async fn respond(
        &self,
        SearchToolProviderInput { query }: SearchToolProviderInput,
    ) -> Result<ToolCallResult<SearchToolProviderOutput>, ProviderError> {
        let Some(search_index_reader) = self.search_index_reader_holder.get().await else {
            return Ok(ToolCallErrorMessage(
                "Search index is not ready yet. There are no successful builds yet, or the server needs more time to start.",
            )
            .into());
        };
        let search_index_found_documents = spawn_blocking(move || {
            search_index_reader.query(SearchIndexQueryParams {
                offset: 0,
                per_page: SEARCH_RESULTS_PER_PAGE,
                query,
            })
        })
        .await??;

        Ok(ToolCallResult::Success(ToolCallSuccess {
            content: search_index_found_documents
                .iter()
                .map(|search_index_found_document| self.resource_link(search_index_found_document))
                .collect(),
            structured_content: SearchToolProviderOutput {},
        }))
    }
}

#[cfg(test)]
mod tests {
    use std::path::Path;
    use std::sync::Arc;

    use anyhow::Result;
    use poet_assets::asset_path_renderer::AssetPathRenderer;
    use poet_content::build_authors::build_authors;
    use poet_content::build_project::build_project;
    use poet_content::build_project_params::BuildProjectParams;
    use poet_content::build_project_result_stub::BuildProjectResultStub;
    use poet_filesystem::filesystem::Filesystem as _;
    use poet_filesystem::storage::Storage;
    use poet_mcp::provider_error::ProviderError;
    use poet_mcp::tool_call_result::ToolCallResult;
    use poet_mcp::tool_call_success::ToolCallSuccess;
    use poet_mcp::tool_provider::ToolProvider as _;
    use poet_mcp::tool_responder::ToolResponder as _;
    use tempfile::tempdir;

    use crate::compile_poet_shortcodes::compile_poet_shortcodes;
    use crate::holder::Holder as _;
    use crate::search_index::SearchIndex;
    use crate::search_index_reader_holder::SearchIndexReaderHolder;
    use crate::search_tool::SearchTool;
    use crate::search_tool::SearchToolProviderInput;

    fn empty_search_tool() -> SearchTool {
        SearchTool {
            mcp_resource_provider_content_documents: Default::default(),
            search_index_reader_holder: Default::default(),
        }
    }

    async fn search_tool_with_index() -> Result<SearchTool> {
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
                "+++\ndescription = \"Guide\"\nlayout = \"Layout\"\ntitle = \"Guide\"\n+++\n\nkeyword zebra body\n",
            )
            .await?;

        let rhai_template_renderer = compile_poet_shortcodes(&source_filesystem).await?;
        let authors = build_authors(source_filesystem.as_ref()).await?;

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
            source_filesystem: source_filesystem.as_ref(),
        })
        .await?;

        let search_index_reader =
            SearchIndex::create_in_memory(content_document_sources).index()?;
        let search_index_reader_holder = SearchIndexReaderHolder::default();

        search_index_reader_holder
            .set(Some(Arc::new(search_index_reader)))
            .await;

        Ok(SearchTool {
            mcp_resource_provider_content_documents: Default::default(),
            search_index_reader_holder,
        })
    }

    #[test]
    fn tool_name_is_search() {
        assert_eq!(empty_search_tool().name(), "search");
    }

    #[tokio::test]
    async fn responds_with_failure_when_index_not_ready() -> Result<(), ProviderError> {
        let result = empty_search_tool()
            .respond(SearchToolProviderInput {
                query: "anything".to_string(),
            })
            .await?;

        assert!(matches!(result, ToolCallResult::Failure(_)));

        Ok(())
    }

    #[tokio::test]
    async fn responds_with_resource_links_for_matches() -> Result<(), ProviderError> {
        let result = search_tool_with_index()
            .await?
            .respond(SearchToolProviderInput {
                query: "zebra".to_string(),
            })
            .await?;

        assert!(matches!(
            result,
            ToolCallResult::Success(ToolCallSuccess { content, .. }) if content.len() == 1
        ));

        Ok(())
    }
}
