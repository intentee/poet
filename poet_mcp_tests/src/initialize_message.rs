use poet_mcp::mcp_protocol_version::MCP_PROTOCOL_VERSION;
use serde_json::Value;
use serde_json::json;

#[must_use]
pub fn initialize_message(client_capabilities: &Value) -> Value {
    json!({
        "id": 1,
        "jsonrpc": "2.0",
        "method": "initialize",
        "params": {
            "capabilities": client_capabilities,
            "clientInfo": { "name": "client", "version": "1.0.0" },
            "protocolVersion": MCP_PROTOCOL_VERSION,
        },
    })
}
