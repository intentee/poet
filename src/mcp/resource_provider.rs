use std::sync::Arc;

use actix_web::rt;
use anyhow::Result;
use anyhow::anyhow;
use async_trait::async_trait;
use log::error;
use log::warn;
use tokio::sync::Notify;
use tokio::sync::mpsc;
use tokio::sync::mpsc::Receiver;
use tokio_util::sync::CancellationToken;

use crate::mcp::resource::Resource;
use crate::mcp::resource_content_parts::ResourceContentParts;
use crate::mcp::resource_provider_list_params::ResourceProviderListParams;
use crate::mcp::resource_reference::ResourceReference;
use crate::mcp::resource_template_provider::ResourceTemplateProvider;

#[async_trait]
pub trait ResourceProvider: ResourceTemplateProvider + Send + Sync + 'static {
    async fn list_resources(&self, params: ResourceProviderListParams) -> Result<Vec<Resource>>;

    async fn read_resource_contents(
        &self,
        resource_reference: ResourceReference,
    ) -> Result<Option<ResourceContentParts>>;

    async fn resource_update_notifier(
        &self,
        resource_reference: ResourceReference,
    ) -> Result<Option<Arc<Notify>>>;

    fn total(&self) -> usize;

    fn resource_uri(&self, resource_path: &str) -> String {
        format!("{}/{resource_path}", self.resource_uri_prefix())
    }

    async fn subscribe(
        self: Arc<Self>,
        cancellation_token: CancellationToken,
        resource_reference: ResourceReference,
    ) -> Result<Option<Receiver<ResourceContentParts>>> {
        let (resource_content_parts_tx, resource_content_parts_rx) = mpsc::channel(3);
        let resource_update_notifier: Arc<Notify> = self
            .resource_update_notifier(resource_reference.clone())
            .await?
            .ok_or_else(|| {
                anyhow!(
                    "Unable to obtain resource update notifier for resource: '{}'",
                    resource_reference.path
                )
            })?;
        let this = self.clone();

        rt::spawn(async move {
            loop {
                tokio::select! {
                    _ = cancellation_token.cancelled() => {
                        break;
                    }
                    _ = resource_update_notifier.notified() => {
                        let notification = match this.read_resource_contents(resource_reference.clone()).await {
                            Ok(Some(resource_content_parts)) => resource_content_parts,
                            Ok(None) => {
                                warn!("Resource has been removed while being subscribed to: '{}'", resource_reference.path);
                                break;
                            }
                            Err(err) => {
                                error!("Unable to get resource content parts for '{}': {err:#?}", resource_reference.path);
                                break;
                            }
                        };

                        if let Err(err) = resource_content_parts_tx.send(notification).await {
                            error!("Unable to forward resource update: {err:#?}");
                        }
                    }
                }
            }
        });

        Ok(Some(resource_content_parts_rx))
    }
}
