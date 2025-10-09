pub mod http_server;
pub mod project_builder;
pub mod search_index_builder;
pub mod shortcodes_compiler;

use anyhow::Result;
use async_trait::async_trait;

#[async_trait]
pub trait Service {
    async fn run(&self) -> Result<()>;
}
