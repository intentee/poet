use crate::mcp::list_resources_cursor::ListResourcesCursor;

pub struct ListResourcesParams {
    pub cursor: ListResourcesCursor,
    pub per_page: usize,
}
