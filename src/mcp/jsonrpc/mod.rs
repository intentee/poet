pub mod client_to_server_message;
pub mod empty_object;
pub mod id;
pub mod implementation;
pub mod meta;
pub mod notification;
pub mod request;
pub mod response;
pub mod role;
pub mod serde_defaults;
pub mod server_to_client_notification;
pub mod server_to_client_response;

pub const JSONRPC_VERSION: &str = "2.0";
