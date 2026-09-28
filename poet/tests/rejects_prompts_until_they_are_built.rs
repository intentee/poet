use poet::holder::Holder;
use poet::mcp_prompt_provider_prompt_documents::McpPromptProviderPromptDocuments;
use poet::poet_error::PoetError;
use poet_mcp::list_resources_cursor::ListResourcesCursor;
use poet_mcp::prompt_provider::PromptProvider as _;

#[tokio::test]
async fn rejects_prompts_until_they_are_built() {
    let Err(provider_error) = McpPromptProviderPromptDocuments {
        prompt_document_controller_collection_holder: Holder::default(),
    }
    .list_prompts(ListResourcesCursor {
        offset: 0,
        per_page: 1,
    })
    .await
    else {
        panic!("expected prompts to be unavailable");
    };

    assert!(matches!(
        provider_error.downcast_ref::<PoetError>(),
        Some(PoetError::PromptDocumentControllerCollectionNotReady)
    ));
}
