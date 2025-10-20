use anyhow::Result;
use async_trait::async_trait;
use schemars::JsonSchema;
use serde::Deserialize;
use serde::Serialize;
use tokio::task::spawn_blocking;

use crate::content_document_front_matter::ContentDocumentFrontMatter;
use crate::content_document_reference::ContentDocumentReference;
use crate::holder::Holder;
use crate::mcp::jsonrpc::content_block::ContentBlock;
use crate::mcp::jsonrpc::content_block::resource_link::ResourceLink;
use crate::mcp::jsonrpc::response::success::tool_call_result::ToolCallResult;
use crate::mcp::jsonrpc::response::success::tool_call_result::success::Success;
use crate::mcp::resource_provider::ResourceProvider as _;
use crate::mcp::tool_call_error_message::ToolCallErrorMesage;
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
                ToolCallErrorMesage(
                    "Search index is not ready yet. There are no successful builds yet, or the server needs more time to start."
                ).into(),
            ),
        }
    }
}
