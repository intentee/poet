use std::net::SocketAddr;
use std::path::PathBuf;

use serde::Deserialize;

#[derive(Deserialize)]
pub struct PoetConfig {
    pub static_files_directory: PathBuf,
    pub watch_server_addr: SocketAddr,
}
