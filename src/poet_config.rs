use std::net::SocketAddr;
use std::path::PathBuf;

use serde::Deserialize;

#[derive(Clone, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PoetConfig {
    pub static_files_directory: PathBuf,
    pub static_files_public_path: String,
    pub watch_server_addr: SocketAddr,
}
