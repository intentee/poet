use async_trait::async_trait;

use crate::provider_error::ProviderError;
use crate::tool_call_result::ToolCallResult;
use crate::tool_provider::ToolProvider;

#[async_trait]
pub trait ToolResponder<TToolProvider>
where
    Self: Send + Sync,
    TToolProvider: ToolProvider,
{
    async fn respond(
        &self,
        input: <TToolProvider as ToolProvider>::Input,
    ) -> Result<ToolCallResult<<TToolProvider as ToolProvider>::Output>, ProviderError>;
}
