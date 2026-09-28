use std::collections::BTreeMap;

use async_trait::async_trait;
use poet_mcp::list_resources_cursor::ListResourcesCursor;
use poet_mcp::prompt::Prompt;
use poet_mcp::prompt_message::PromptMessage;
use poet_mcp::prompt_provider::PromptProvider;
use poet_mcp::prompts_get_request::PromptsGetRequest;
use poet_mcp::prompts_get_result::PromptsGetResult;
use poet_mcp::provider_error::ProviderError;

pub struct InMemoryPromptProvider {
    pub prompt_messages: BTreeMap<String, Vec<PromptMessage>>,
}

#[async_trait]
impl PromptProvider for InMemoryPromptProvider {
    async fn get_prompt(
        &self,
        request: PromptsGetRequest,
    ) -> Result<Option<PromptsGetResult>, ProviderError> {
        Ok(self
            .prompt_messages
            .get(&request.params.name)
            .map(|messages| PromptsGetResult {
                description: None,
                messages: messages.clone(),
                meta: None,
            }))
    }

    async fn list_prompts(
        &self,
        ListResourcesCursor { offset, per_page }: ListResourcesCursor,
    ) -> Result<Vec<Prompt>, ProviderError> {
        Ok(self
            .prompt_messages
            .keys()
            .skip(offset)
            .take(per_page)
            .map(|prompt_name| Prompt {
                arguments: vec![],
                description: prompt_name.clone(),
                name: prompt_name.clone(),
                title: prompt_name.clone(),
            })
            .collect())
    }
}
