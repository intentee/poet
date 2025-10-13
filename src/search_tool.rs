use anyhow::Result;
use async_trait::async_trait;
use schemars::JsonSchema;
use serde::Deserialize;
use serde::Serialize;

use crate::mcp::tool_provider::ToolProvider;
use crate::mcp::tool_responder::ToolResponder;

#[derive(Deserialize, JsonSchema, Serialize)]
pub struct SearchToolProviderInputSchema {}

#[derive(Deserialize, JsonSchema, Serialize)]
pub struct SearchToolProviderOutputSchema {}

pub struct SearchTool {}

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
    ) -> Result<SearchToolProviderOutputSchema> {
        unimplemented!()
    }
}
