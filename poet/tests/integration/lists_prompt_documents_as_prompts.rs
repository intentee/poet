use poet_mcp::list_resources_cursor::ListResourcesCursor;
use poet_mcp::prompt::Prompt;
use poet_mcp::prompt_provider::PromptProvider as _;
use poet_mcp::provider_error::ProviderError;

use crate::prompt_provider_with_fixture_prompt::prompt_provider_with_fixture_prompt;

#[tokio::test]
async fn lists_prompt_documents_as_prompts() -> Result<(), ProviderError> {
    assert!(matches!(
        prompt_provider_with_fixture_prompt()
            .await?
            .list_prompts(ListResourcesCursor {
                offset: 0,
                per_page: 1,
            })
            .await?
            .as_slice(),
        [Prompt { name, title, .. }] if name == "greet" && title == "Greet"
    ));

    Ok(())
}
