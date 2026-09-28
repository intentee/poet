use std::collections::HashMap;

use poet_mcp::jsonrpc_version::JSONRPC_VERSION;
use poet_mcp::prompt_provider::PromptProvider as _;
use poet_mcp::prompts_get_request::PromptsGetRequest;
use poet_mcp::prompts_get_request_params::PromptsGetRequestParams;
use poet_mcp::provider_error::ProviderError;
use poet_mcp::request_id::RequestId;

use crate::prompt_provider_with_fixture_prompt::prompt_provider_with_fixture_prompt;

#[tokio::test]
async fn ignores_unknown_prompt() -> Result<(), ProviderError> {
    assert!(
        prompt_provider_with_fixture_prompt()
            .await?
            .get_prompt(PromptsGetRequest {
                id: RequestId::Number(1),
                jsonrpc: JSONRPC_VERSION.to_owned(),
                params: PromptsGetRequestParams {
                    arguments: HashMap::new(),
                    meta: None,
                    name: "missing".to_owned(),
                },
            })
            .await?
            .is_none()
    );

    Ok(())
}
