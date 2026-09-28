use std::sync::Arc;

use tokio::sync::Notify;
use tokio_util::sync::CancellationToken;

use crate::mcp_error::McpError;
use crate::resource_content::ResourceContent;
use crate::resource_provider::ResourceProvider;
use crate::resource_reference::ResourceReference;

pub struct ResolvedResource {
    pub provider: Arc<dyn ResourceProvider>,
    pub resource_reference: ResourceReference,
}

impl ResolvedResource {
    pub async fn read_contents(self) -> Result<Vec<ResourceContent>, McpError> {
        let uri = self.resource_reference.uri_string.clone();

        self.provider
            .read_resource_contents(self.resource_reference)
            .await
            .map_err(|source| McpError::ResourceProviderFailed {
                uri: uri.clone(),
                source,
            })?
            .ok_or(McpError::ResourceNotFound { uri })
    }

    #[must_use]
    pub fn update_notifier(self, cancellation_token: CancellationToken) -> Arc<Notify> {
        self.provider
            .resource_update_notifier(cancellation_token, self.resource_reference)
    }
}
