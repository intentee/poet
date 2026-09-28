pub mod builds_project;
pub mod cancel_on_termination_signal;
pub mod handler;
pub mod make;
pub mod respond_with_generated_page;
pub mod respond_with_generated_page_holder;
pub mod serve;
pub mod service;
pub mod service_manager;
pub mod value_parser;
pub mod watch;

const HTTP_SERVER_SHUTDOWN_TIMEOUT_SECONDS: u64 = 1;
const MCP_STREAMABLE_HTTP_MOUNT_PATH: &str = "/mcp/streamable";
const STATIC_FILES_PUBLIC_PATH: &str = "assets";
