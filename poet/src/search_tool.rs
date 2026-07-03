use anyhow::Result;
use async_trait::async_trait;
use schemars::JsonSchema;
use serde::Deserialize;
use serde::Serialize;
use tokio::task::spawn_blocking;

use crate::content_document_front_matter::ContentDocumentFrontMatter;
use crate::content_document_reference::ContentDocumentReference;
use crate::holder::Holder;
use crate::mcp::content_block::ContentBlock;
use crate::mcp::content_block::resource_link::ResourceLink;
use crate::mcp::jsonrpc::response::success::tool_call_result::ToolCallResult;
use crate::mcp::jsonrpc::response::success::tool_call_result::success::Success;
use crate::mcp::resource_provider::ResourceProvider as _;
use crate::mcp::tool_call_error_message::ToolCallErrorMessage;
use crate::mcp::tool_provider::ToolProvider;
use crate::mcp::tool_responder::ToolResponder;
use crate::mcp_resource_provider_content_documents::McpResourceProviderContentDocuments;
use crate::search_index_found_document::SearchIndexFoundDocument;
use crate::search_index_query_params::SearchIndexQueryParams;
use crate::search_index_reader_holder::SearchIndexReaderHolder;

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

impl ToolProvider for SearchTool {
    type Input = SearchToolProviderInput;
    type Output = SearchToolProviderOutput;

    fn name(&self) -> String {
        "search".to_string()
    }
}

#[async_trait]
impl ToolResponder<Self> for SearchTool {
    async fn respond(
        &self,
        SearchToolProviderInput { query }: SearchToolProviderInput,
    ) -> Result<ToolCallResult<SearchToolProviderOutput>> {
        match self
            .search_index_reader_holder
            .get()
            .await {
            Some(search_index_reader) => {
                let search_index_found_documents: Vec<SearchIndexFoundDocument> = spawn_blocking(move || {
                        search_index_reader.query(SearchIndexQueryParams {
                            cursor: Default::default(),
                            query,
                        })
                    })
                    .await??;

                Ok(ToolCallResult::Success(Success {
                    content: search_index_found_documents
                        .iter()
                        .map(|SearchIndexFoundDocument {
                            content_document_reference: content_document_reference @ ContentDocumentReference {
                                front_matter: ContentDocumentFrontMatter {
                                    description,
                                    title,
                                    ..
                                },
                                ..
                            }
                        }| ContentBlock::ResourceLink(ResourceLink {
                            description: Some(description.to_string()),
                            mime_type: Some("text/markdown".to_string()),
                            name: title.to_string(),
                            title: Some(title.to_string()),
                            uri: self.mcp_resource_provider_content_documents.resource_uri(&content_document_reference.basename().to_string()),
                        }))
                        .collect()
                    ,
                    structured_content: SearchToolProviderOutput {
                    }
                }))
            },
            None => Ok(
                ToolCallErrorMessage(
                    "Search index is not ready yet. There are no successful builds yet, or the server needs more time to start."
                ).into(),
            ),
        }
    }
}

#[cfg(test)]
mod tests {
    use std::path::Path;
    use std::sync::Arc;

    use tempfile::tempdir;

    use super::*;
    use crate::asset_path_renderer::AssetPathRenderer;
    use crate::build_authors::build_authors;
    use crate::build_project::build_project;
    use crate::build_project::build_project_params::BuildProjectParams;
    use crate::build_project::build_project_result_stub::BuildProjectResultStub;
    use crate::compile_shortcodes::compile_shortcodes;
    use crate::filesystem::Filesystem as _;
    use crate::filesystem::storage::Storage;
    use crate::search_index::SearchIndex;

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
    async fn responds_with_failure_when_index_not_ready() -> Result<()> {
        let result = empty_search_tool()
            .respond(SearchToolProviderInput {
                query: "anything".to_string(),
            })
            .await?;

        assert!(matches!(result, ToolCallResult::Failure(_)));

        Ok(())
    }

    #[tokio::test]
    async fn responds_with_resource_links_for_matches() -> Result<()> {
        let result = search_tool_with_index()
            .await?
            .respond(SearchToolProviderInput {
                query: "zebra".to_string(),
            })
            .await?;

        match result {
            ToolCallResult::Success(success) => assert_eq!(success.content.len(), 1),
            ToolCallResult::Failure(_) => unreachable!("expected a successful search result"),
        }

        Ok(())
    }
}
