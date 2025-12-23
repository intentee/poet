use anyhow::Result;
use async_trait::async_trait;

use crate::mcp::jsonrpc::request::prompts_get::PromptsGet;
use crate::mcp::jsonrpc::response::success::prompts_get_result::PromptsGetResult;
use crate::mcp::prompt::Prompt;

#[async_trait]
pub trait PromptController: Send + Sync {
    fn get_mcp_prompt(&self) -> Prompt;

    async fn respond_to(&self, request: PromptsGet) -> Result<PromptsGetResult>;
}
