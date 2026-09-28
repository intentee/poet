use anyhow::Result;
use async_trait::async_trait;

use poet_mcp::prompt::Prompt;
use poet_mcp::prompts_get_request::PromptsGetRequest;
use poet_mcp::prompts_get_result::PromptsGetResult;

#[async_trait]
pub trait PromptController: Send + Sync {
    fn get_mcp_prompt(&self) -> Prompt;

    async fn respond_to(&self, request: PromptsGetRequest) -> Result<PromptsGetResult>;
}
