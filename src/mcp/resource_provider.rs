use anyhow::Result;
use async_trait::async_trait;
use tokio::sync::mpsc::Receiver;

use crate::mcp::resource::Resource;
use crate::mcp::resource_content_parts::ResourceContentParts;
use crate::mcp::resource_provider_list_params::ResourceProviderListParams;
use crate::mcp::resource_reference::ResourceReference;
use crate::mcp::resource_template_provider::ResourceTemplateProvider;

#[async_trait]
pub trait ResourceProvider: ResourceTemplateProvider + Send + Sync {
    async fn list_resources(&self, params: ResourceProviderListParams) -> Result<Vec<Resource>>;

    async fn read_resource_contents(
        &self,
        resource_reference: ResourceReference,
    ) -> Result<Option<ResourceContentParts>>;

    async fn subscribe(
        &self,
        resource_reference: ResourceReference,
    ) -> Result<Option<Receiver<ResourceContentParts>>>;

    fn total(&self) -> usize;

    fn resource_uri(&self, resource_path: &str) -> String {
        format!("{}/{resource_path}", self.resource_uri_prefix())
    }
}
