use poet::holder::Holder;
use poet::mcp_resource_provider_content_documents::McpResourceProviderContentDocuments;
use poet::search_tool::SearchTool;
use poet_mcp::tool_provider::ToolProvider as _;

#[test]
fn search_tool_is_named_search() {
    assert_eq!(
        SearchTool {
            mcp_resource_provider_content_documents: McpResourceProviderContentDocuments::default(),
            search_index_reader_holder: Holder::default(),
        }
        .name(),
        "search"
    );
}
