use poet_mcp::list_resources_cursor::ListResourcesCursor;
use poet_mcp::mcp_error::McpError;
use poet_mcp_tests::listed_resource_uris::listed_resource_uris;

#[actix_web::test]
async fn stops_listing_once_page_is_full() -> Result<(), McpError> {
    assert_eq!(
        listed_resource_uris(ListResourcesCursor {
            offset: 0,
            per_page: 1,
        })
        .await?,
        vec!["memory://articles/first"]
    );

    Ok(())
}
