use anyhow::Result;
use async_trait::async_trait;

use crate::mcp::resource::Resource;
use crate::mcp::resource_provider_list_params::ResourceProviderListParams;

#[async_trait]
pub trait ResourceProvider: Send + Sync {
    fn id(&self) -> String;

    async fn list_resources(&self, params: ResourceProviderListParams) -> Result<Vec<Resource>>;

    fn total(&self) -> usize;
}
