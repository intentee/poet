use serde_json::Value;

pub enum ToolCallResult {
    NotFound,
    Success(Value),
}
