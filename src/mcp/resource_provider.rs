use anyhow::Result;
use async_trait::async_trait;

use crate::mcp::jsonrpc::response::success::resources_read::ResourceContent;
use crate::mcp::resource::Resource;
use crate::mcp::resource_provider_list_params::ResourceProviderListParams;

#[async_trait]
pub trait ResourceProvider: Send + Sync {
    async fn list_resources(&self, params: ResourceProviderListParams) -> Result<Vec<Resource>>;

    async fn read_resource_contents(
        &self,
        resource_uri: String,
        resource_path: String,
    ) -> Result<Option<Vec<ResourceContent>>>;

    fn resource_class(&self) -> String;

    fn total(&self) -> usize;

    fn resource_uri(&self, resource_path: &str) -> String {
        format!("{}://{resource_path}", self.resource_class())
    }
}
