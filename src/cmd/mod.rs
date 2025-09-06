mod builds_project;
pub mod generate;
mod value_parser;
pub mod watch;

use anyhow::Result;
use async_trait::async_trait;

#[async_trait]
pub trait Handler {
    async fn handle(&self) -> Result<()>;
}
