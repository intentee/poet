use anyhow::Result;
use async_trait::async_trait;
use schemars::JsonSchema;
use serde::Deserialize;
use serde::Serialize;

use crate::mcp::jsonrpc::response::success::tool_call_result::ToolCallResult;
use crate::mcp::tool_provider::ToolProvider;
use crate::mcp::tool_responder::ToolResponder;
use crate::search_index_reader_holder::SearchIndexReaderHolder;

#[derive(Deserialize, JsonSchema, Serialize)]
pub struct SearchToolProviderInputSchema {
    pub query: String,
}

#[derive(Deserialize, JsonSchema, Serialize)]
pub struct SearchToolProviderOutputSchema {}

pub struct SearchTool {
    pub search_index_reader_holder: SearchIndexReaderHolder,
}

impl ToolProvider for SearchTool {
    type InputSchema = SearchToolProviderInputSchema;
    type OutputSchema = SearchToolProviderOutputSchema;

    fn name(&self) -> String {
        "search".to_string()
    }
}

#[async_trait]
impl ToolResponder<Self> for SearchTool {
    async fn respond(
        &self,
        input: SearchToolProviderInputSchema,
    ) -> Result<ToolCallResult<SearchToolProviderOutputSchema>> {
        unimplemented!()
    }
}
