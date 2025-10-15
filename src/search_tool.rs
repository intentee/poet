use anyhow::Result;
use async_trait::async_trait;
use schemars::JsonSchema;
use serde::Deserialize;
use serde::Serialize;
use tokio::task::spawn_blocking;

use crate::holder::Holder;
use crate::mcp::jsonrpc::content_block::ContentBlock;
use crate::mcp::jsonrpc::content_block::text_content::TextContent;
use crate::mcp::jsonrpc::response::success::tool_call_result::ToolCallResult;
use crate::mcp::tool_call_error_message::ToolCallErrorMesage;
use crate::mcp::tool_provider::ToolProvider;
use crate::mcp::tool_responder::ToolResponder;
use crate::search_index_reader_holder::SearchIndexReaderHolder;

#[derive(Deserialize, JsonSchema, Serialize)]
pub struct SearchToolProviderInput {
    pub query: String,
}

#[derive(Deserialize, JsonSchema, Serialize)]
pub struct SearchToolProviderOutput {}

pub struct SearchTool {
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
            Some(search_index_reader) => Ok(ToolCallResult::Success {
                content: spawn_blocking(move || -> Result<Vec<ContentBlock>> {
                        Ok(search_index_reader
                            .query(&query)?
                            .into_iter()
                            .map(|text| ContentBlock::TextContent(TextContent {
                                text,
                            }))
                            .collect())
                    })
                    .await??,
                structured_content: SearchToolProviderOutput {
                }
            }),
            None => Ok(
                ToolCallErrorMesage(
                    "Search index is not ready yet. There are no successful builds yet, or the server needs more time to start."
                ).into(),
            ),
        }
    }
}
