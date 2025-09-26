use anyhow::Result;
use async_trait::async_trait;

use crate::mcp::resource::Resource;

#[async_trait]
pub trait ResourceProvider: Send + Sync {
    fn id(&self) -> String;

    async fn list_resources(&self, offset: usize, limit: usize) -> Result<Vec<Resource>>;

    fn total(&self) -> usize;
}
