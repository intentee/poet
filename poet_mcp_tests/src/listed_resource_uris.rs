use poet_mcp::list_resources_cursor::ListResourcesCursor;
use poet_mcp::mcp_error::McpError;
use poet_mcp::resource::Resource;
use poet_mcp::resource_list_aggregate::ResourceListAggregate;

use crate::resource_provider_with_paths::resource_provider_with_paths;

pub async fn listed_resource_uris(
    list_resources_cursor: ListResourcesCursor,
) -> Result<Vec<String>, McpError> {
    Ok(ResourceListAggregate::from(vec![
        resource_provider_with_paths("articles", &["first", "second"]),
        resource_provider_with_paths("notes", &["third", "fourth"]),
    ])
    .list_resources(list_resources_cursor)
    .await?
    .into_iter()
    .map(|Resource { uri, .. }| uri)
    .collect())
}
