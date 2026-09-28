use poet_mcp::list_resources_cursor::ListResourcesCursor;
use poet_mcp::mcp_error::McpError;
use poet_mcp_tests::listed_resource_uris::listed_resource_uris;

#[actix_web::test]
async fn lists_resources_spanning_two_providers() -> Result<(), McpError> {
    assert_eq!(
        listed_resource_uris(ListResourcesCursor {
            offset: 1,
            per_page: 2,
        })
        .await?,
        vec!["memory://articles/second", "memory://notes/fourth"]
    );

    Ok(())
}
