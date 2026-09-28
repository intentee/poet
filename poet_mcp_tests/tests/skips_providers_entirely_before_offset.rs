use poet_mcp::list_resources_cursor::ListResourcesCursor;
use poet_mcp::mcp_error::McpError;
use poet_mcp_tests::listed_resource_uris::listed_resource_uris;

#[actix_web::test]
async fn skips_providers_entirely_before_offset() -> Result<(), McpError> {
    assert_eq!(
        listed_resource_uris(ListResourcesCursor {
            offset: 3,
            per_page: 2,
        })
        .await?,
        vec!["memory://notes/third"]
    );

    Ok(())
}
