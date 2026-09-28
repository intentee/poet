use std::sync::Arc;

use async_trait::async_trait;
use tokio::sync::Notify;
use tokio_util::sync::CancellationToken;

use crate::provider_error::ProviderError;
use crate::resource::Resource;
use crate::resource_content::ResourceContent;
use crate::resource_provider_list_params::ResourceProviderListParams;
use crate::resource_reference::ResourceReference;
use crate::resource_template_provider::ResourceTemplateProvider;

#[async_trait]
pub trait ResourceProvider: ResourceTemplateProvider + Send + Sync + 'static {
    async fn list_resources(
        &self,
        params: ResourceProviderListParams,
    ) -> Result<Vec<Resource>, ProviderError>;

    async fn read_resource_contents(
        &self,
        resource_reference: ResourceReference,
    ) -> Result<Option<Vec<ResourceContent>>, ProviderError>;

    fn resource_update_notifier(
        self: Arc<Self>,
        cancellation_token: CancellationToken,
        resource_reference: ResourceReference,
    ) -> Arc<Notify>;

    fn total(&self) -> usize;

    fn resource_uri(&self, resource_path: &str) -> String {
        format!("{}/{resource_path}", self.resource_uri_prefix())
    }
}
