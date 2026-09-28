use std::sync::Arc;

use poet::holder::Holder;
use poet::mcp_resource_provider_content_documents::McpResourceProviderContentDocuments;
use poet::search_tool::SearchTool;
use poet::search_tool::SearchToolProviderInput;
use poet_mcp::content_block::ContentBlock;
use poet_mcp::provider_error::ProviderError;
use poet_mcp::resource_link::ResourceLink;
use poet_mcp::tool_call_result::ToolCallResult;
use poet_mcp::tool_call_success::ToolCallSuccess;
use poet_mcp::tool_responder::ToolResponder as _;
use poet_search_tests::index_fixture_document::index_fixture_document;

#[tokio::test]
async fn search_tool_links_matching_resources() -> Result<(), ProviderError> {
    let search_index_reader_holder = Holder::default();

    search_index_reader_holder.set(Arc::new(
        index_fixture_document("Guide description", "keyword zebra body").await?,
    ));

    assert!(matches!(
        SearchTool {
            mcp_resource_provider_content_documents: McpResourceProviderContentDocuments::default(),
            search_index_reader_holder,
        }
        .respond(SearchToolProviderInput {
            query: "zebra".to_owned(),
        })
        .await?,
        ToolCallResult::Success(ToolCallSuccess { content, .. }) if matches!(
            content.as_slice(),
            [ContentBlock::ResourceLink(ResourceLink { name, uri, .. })] if name == "Guide" && uri == "poet://content/guide"
        )
    ));

    Ok(())
}
