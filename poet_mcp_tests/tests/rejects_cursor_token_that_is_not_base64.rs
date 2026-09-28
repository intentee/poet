use poet_mcp::list_resources_cursor::ListResourcesCursor;
use serde_json::error::Category;
use serde_json::from_value;
use serde_json::json;

#[test]
fn rejects_cursor_token_that_is_not_base64() {
    assert!(matches!(
        from_value::<ListResourcesCursor>(json!("@@@")),
        Err(json_error) if json_error.classify() == Category::Data
    ));
}
