pub mod accepts_all;
pub mod mcp_http_service;
pub mod mcp_http_service_factory;
pub mod mcp_responder;
pub mod mcp_responder_context;
pub mod mcp_responder_handler;
pub mod session;
pub mod session_manager;
pub mod session_storage;

pub const MCP_PROTOCOL_VERSION: &str = "2025-06-18";
pub const MCP_SESSION_HEADER_NAME: &str = "Mcp-Session-Id";
