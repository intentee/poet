use crate::mcp::list_resources_cursor::ListResourcesCursor;

pub struct SearchIndexQueryParams {
    pub cursor: ListResourcesCursor,
    pub query: String,
}
