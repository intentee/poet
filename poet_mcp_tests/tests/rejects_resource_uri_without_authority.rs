use poet_mcp::mcp_error::McpError;
use poet_mcp::resource_reference::ResourceReference;

#[test]
fn rejects_resource_uri_without_authority() {
    assert!(matches!(
        ResourceReference::try_from("/guide"),
        Err(McpError::MissingResourceUriAuthority { uri }) if uri == "/guide"
    ));
}
