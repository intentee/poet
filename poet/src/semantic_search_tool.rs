use std::sync::Arc;

use anyhow::Result;
use async_trait::async_trait;
use paddler_types::embedding_input_document::EmbeddingInputDocument;
use schemars::JsonSchema;
use serde::Deserialize;
use serde::Serialize;

use crate::generate_embedding::paddler_embedding_client::PaddlerEmbeddingClient;
use crate::mcp::content_block::ContentBlock;
use crate::mcp::content_block::resource_link::ResourceLink;
use crate::mcp::jsonrpc::response::success::tool_call_result::ToolCallResult;
use crate::mcp::jsonrpc::response::success::tool_call_result::success::Success;
use crate::mcp::resource_provider::ResourceProvider as _;
use crate::mcp::resource_provider_list_params::ResourceProviderListParams;
use crate::mcp::tool_call_error_message::ToolCallErrorMessage;
use crate::mcp::tool_provider::ToolProvider;
use crate::mcp::tool_responder::ToolResponder;
use crate::mcp_resource_provider_content_documents::McpResourceProviderContentDocuments;
use crate::semantic_search_index::SemanticSearchIndex;

const MIN_SCORE: f32 = 0.0;
const TOP_K: usize = 10;

#[derive(Deserialize, JsonSchema, Serialize)]
pub struct SemanticSearchToolInput {
    pub query: String,
}

#[derive(Deserialize, JsonSchema, Serialize)]
pub struct SemanticSearchToolOutput {}

pub struct SemanticSearchTool {
    pub mcp_resource_provider_content_documents: McpResourceProviderContentDocuments,
    pub paddler_embeddings_client: Arc<PaddlerEmbeddingClient>,
    pub semantic_search_index: Arc<SemanticSearchIndex>,
}

impl ToolProvider for SemanticSearchTool {
    type Input = SemanticSearchToolInput;
    type Output = SemanticSearchToolOutput;

    fn name(&self) -> String {
        "semantic_search".to_string()
    }

    fn description(&self) -> Option<String> {
        Some("Search content using semantic similarity".to_string())
    }
}

#[async_trait]
impl ToolResponder<Self> for SemanticSearchTool {
    async fn respond(
        &self,
        SemanticSearchToolInput { query }: SemanticSearchToolInput,
    ) -> Result<ToolCallResult<SemanticSearchToolOutput>> {
        let query_embedding = self
            .paddler_embeddings_client
            .generate_embeddings(vec![EmbeddingInputDocument {
                id: "query".to_string(),
                content: query,
            }])
            .await;

        let query_embedding = match query_embedding {
            Ok(embeddings) if !embeddings.is_empty() => embeddings.into_iter().next().unwrap(),
            Ok(_) => {
                return Ok(ToolCallErrorMessage("Embedding service returned no results").into());
            }
            Err(err) => {
                return Ok(
                    ToolCallErrorMessage(&format!("Failed to generate embedding: {err}")).into(),
                );
            }
        };

        let results =
            self.semantic_search_index
                .query(&query_embedding.embedding, TOP_K, MIN_SCORE);

        let resource_list = self
            .mcp_resource_provider_content_documents
            .list_resources(ResourceProviderListParams {
                limit: usize::MAX,
                offset: 0,
            })
            .await?;

        let content: Vec<ContentBlock> = results
            .iter()
            .filter_map(|(basename, _score)| {
                resource_list.iter().find_map(|resource| {
                    if resource.name != *basename {
                        return None;
                    }

                    Some(ContentBlock::ResourceLink(ResourceLink {
                        description: Some(resource.description.clone()),
                        mime_type: Some("text/markdown".to_string()),
                        name: resource.title.clone(),
                        title: Some(resource.title.clone()),
                        uri: self
                            .mcp_resource_provider_content_documents
                            .resource_uri(basename),
                    }))
                })
            })
            .collect();

        Ok(ToolCallResult::Success(Success {
            content,
            structured_content: SemanticSearchToolOutput {},
        }))
    }
}
