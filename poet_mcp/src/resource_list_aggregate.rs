use std::collections::BTreeSet;
use std::sync::Arc;

use crate::list_resources_cursor::ListResourcesCursor;
use crate::mcp_error::McpError;
use crate::resolved_resource::ResolvedResource;
use crate::resource::Resource;
use crate::resource_provider::ResourceProvider;
use crate::resource_provider_list_params::ResourceProviderListParams;
use crate::resource_provider_ordered::ResourceProviderOrdered;
use crate::resource_reference::ResourceReference;
use crate::resource_template::ResourceTemplate;

pub struct ResourceListAggregate {
    pub providers: BTreeSet<ResourceProviderOrdered>,
}

impl ResourceListAggregate {
    #[must_use]
    pub fn total(&self) -> usize {
        self.providers
            .iter()
            .map(|resource_provider| resource_provider.0.total())
            .sum()
    }

    pub async fn list_resources(
        &self,
        ListResourcesCursor { offset, per_page }: ListResourcesCursor,
    ) -> Result<Vec<Resource>, McpError> {
        let mut resources: Vec<Resource> = vec![];
        let mut to_skip = offset;
        let mut to_take = per_page;

        for resource_provider in &self.providers {
            if to_take < 1 {
                break;
            }

            let provider_total = resource_provider.0.total();

            if provider_total < to_skip {
                to_skip -= provider_total;

                continue;
            }

            let mut taken_resources = resource_provider
                .0
                .list_resources(ResourceProviderListParams {
                    limit: to_take.min(provider_total - to_skip),
                    offset: to_skip,
                })
                .await
                .map_err(|source| McpError::ResourceProviderFailed {
                    uri: resource_provider.0.resource_uri_prefix(),
                    source,
                })?;

            to_skip = 0;
            to_take -= taken_resources.len();

            resources.append(&mut taken_resources);
        }

        Ok(resources)
    }

    #[must_use]
    pub fn resource_templates(&self) -> Vec<ResourceTemplate> {
        self.providers
            .iter()
            .map(|resource_provider| resource_provider.0.resource_template())
            .collect()
    }

    pub fn resolve(&self, uri: &str) -> Result<ResolvedResource, McpError> {
        let resource_reference = ResourceReference::try_from(uri)?;

        self.providers
            .iter()
            .find(|resource_provider| resource_provider.0.can_handle(&resource_reference))
            .map(|resource_provider| ResolvedResource {
                provider: resource_provider.0.clone(),
                resource_reference,
            })
            .ok_or_else(|| McpError::NoResourceProvider {
                uri: uri.to_owned(),
            })
    }
}

impl From<Vec<Arc<dyn ResourceProvider>>> for ResourceListAggregate {
    fn from(resource_providers: Vec<Arc<dyn ResourceProvider>>) -> Self {
        Self {
            providers: resource_providers
                .into_iter()
                .map(ResourceProviderOrdered)
                .collect(),
        }
    }
}
