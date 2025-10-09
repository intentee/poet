pub mod http_server;
pub mod project_builder;
pub mod shortcodes_compiler;

use std::sync::Arc;

use anyhow::Result;
use async_trait::async_trait;

#[async_trait]
pub trait Service {
    async fn run(self: Arc<Self>) -> Result<()>;
}
