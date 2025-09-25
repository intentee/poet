use anyhow::Result;
use async_trait::async_trait;

use crate::mcp::resource::Resource;

#[async_trait]
pub trait ProvidesResources: Send + Sync {
    fn total(&self) -> usize;

    async fn list_resources(&self, offset: usize, limit: usize) -> Result<Vec<Resource>>;
}
