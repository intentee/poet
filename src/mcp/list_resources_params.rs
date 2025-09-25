use crate::mcp::list_resources_cursor::ListResourcesCursor;

pub struct ListResourcesParams {
    pub cursor: Option<ListResourcesCursor>,
}
