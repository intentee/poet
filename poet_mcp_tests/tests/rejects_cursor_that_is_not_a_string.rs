use poet_mcp::list_resources_cursor::ListResourcesCursor;
use serde_json::from_value;
use serde_json::json;

#[test]
fn rejects_cursor_that_is_not_a_string() {
    assert!(from_value::<ListResourcesCursor>(json!(5)).is_err());
}
