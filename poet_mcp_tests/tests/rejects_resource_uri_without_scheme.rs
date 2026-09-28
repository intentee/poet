use poet_mcp::mcp_error::McpError;
use poet_mcp::resource_reference::ResourceReference;

#[test]
fn rejects_resource_uri_without_scheme() {
    assert!(matches!(
        ResourceReference::try_from("content:80"),
        Err(McpError::MissingResourceUriScheme { uri }) if uri == "content:80"
    ));
}
