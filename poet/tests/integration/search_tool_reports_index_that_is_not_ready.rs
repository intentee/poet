use poet::holder::Holder;
use poet::mcp_resource_provider_content_documents::McpResourceProviderContentDocuments;
use poet::search_tool::SearchTool;
use poet::search_tool_provider_input::SearchToolProviderInput;
use poet_mcp::provider_error::ProviderError;
use poet_mcp::tool_call_result::ToolCallResult;
use poet_mcp::tool_responder::ToolResponder as _;

#[tokio::test]
async fn search_tool_reports_index_that_is_not_ready() -> Result<(), ProviderError> {
    assert!(matches!(
        SearchTool {
            mcp_resource_provider_content_documents: McpResourceProviderContentDocuments::default(),
            search_index_reader_holder: Holder::default(),
        }
        .respond(SearchToolProviderInput {
            query: "anything".to_owned(),
        })
        .await?,
        ToolCallResult::Failure(_)
    ));

    Ok(())
}
