use std::io::Error as IoError;
use std::io::ErrorKind;
use std::sync::Arc;

use async_trait::async_trait;
use poet_mcp::provider_error::ProviderError;
use poet_mcp::resource::Resource;
use poet_mcp::resource_content::ResourceContent;
use poet_mcp::resource_provider::ResourceProvider;
use poet_mcp::resource_provider_list_params::ResourceProviderListParams;
use poet_mcp::resource_reference::ResourceReference;
use poet_mcp::resource_template_provider::ResourceTemplateProvider;
use tokio::sync::Notify;
use tokio_util::sync::CancellationToken;

pub struct FailingResourceProvider;

impl ResourceTemplateProvider for FailingResourceProvider {
    fn mime_type(&self) -> String {
        "text/plain".to_owned()
    }

    fn resource_class(&self) -> String {
        "failing".to_owned()
    }

    fn resource_scheme(&self) -> String {
        "memory".to_owned()
    }
}

#[async_trait]
impl ResourceProvider for FailingResourceProvider {
    async fn list_resources(
        &self,
        _params: ResourceProviderListParams,
    ) -> Result<Vec<Resource>, ProviderError> {
        Err(IoError::from(ErrorKind::NotConnected).into())
    }

    async fn read_resource_contents(
        &self,
        _resource_reference: ResourceReference,
    ) -> Result<Option<Vec<ResourceContent>>, ProviderError> {
        Err(IoError::from(ErrorKind::NotConnected).into())
    }

    fn resource_update_notifier(
        self: Arc<Self>,
        _cancellation_token: CancellationToken,
        _resource_reference: ResourceReference,
    ) -> Arc<Notify> {
        Arc::default()
    }

    fn total(&self) -> usize {
        1
    }
}
