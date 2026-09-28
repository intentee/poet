use std::collections::BTreeMap;
use std::sync::Arc;

use async_trait::async_trait;
use poet_mcp::provider_error::ProviderError;
use poet_mcp::resource::Resource;
use poet_mcp::resource_content::ResourceContent;
use poet_mcp::resource_provider::ResourceProvider;
use poet_mcp::resource_provider_list_params::ResourceProviderListParams;
use poet_mcp::resource_reference::ResourceReference;
use poet_mcp::resource_template_provider::ResourceTemplateProvider;
use poet_mcp::text_resource_content::TextResourceContent;
use tokio::sync::Notify;
use tokio_util::sync::CancellationToken;

pub struct InMemoryResourceProvider {
    pub resource_class: String,
    pub resource_texts: BTreeMap<String, String>,
    pub resource_update_notifier: Arc<Notify>,
}

impl ResourceTemplateProvider for InMemoryResourceProvider {
    fn mime_type(&self) -> String {
        "text/plain".to_owned()
    }

    fn resource_class(&self) -> String {
        self.resource_class.clone()
    }

    fn resource_scheme(&self) -> String {
        "memory".to_owned()
    }
}

#[async_trait]
impl ResourceProvider for InMemoryResourceProvider {
    async fn list_resources(
        &self,
        ResourceProviderListParams { limit, offset }: ResourceProviderListParams,
    ) -> Result<Vec<Resource>, ProviderError> {
        Ok(self
            .resource_texts
            .iter()
            .skip(offset)
            .take(limit)
            .map(|(resource_path, resource_text)| Resource {
                description: resource_text.clone(),
                name: resource_path.clone(),
                title: resource_path.clone(),
                uri: self.resource_uri(resource_path),
            })
            .collect())
    }

    async fn read_resource_contents(
        &self,
        ResourceReference {
            path, uri_string, ..
        }: ResourceReference,
    ) -> Result<Option<Vec<ResourceContent>>, ProviderError> {
        Ok(self.resource_texts.get(&path).map(|resource_text| {
            vec![ResourceContent::Text(TextResourceContent {
                mime_type: self.mime_type(),
                text: resource_text.clone(),
                uri: uri_string,
            })]
        }))
    }

    fn resource_update_notifier(
        self: Arc<Self>,
        _cancellation_token: CancellationToken,
        _resource_reference: ResourceReference,
    ) -> Arc<Notify> {
        self.resource_update_notifier.clone()
    }

    fn total(&self) -> usize {
        self.resource_texts.len()
    }
}
