use std::cmp;
use std::collections::BTreeSet;
use std::sync::Arc;

use anyhow::Context as _;
use anyhow::Result;
use anyhow::anyhow;
use http::Uri;
use log::warn;
use tokio::sync::mpsc::Receiver;

use crate::mcp::list_resources_cursor::ListResourcesCursor;
use crate::mcp::list_resources_params::ListResourcesParams;
use crate::mcp::resource::Resource;
use crate::mcp::resource_content_parts::ResourceContentParts;
use crate::mcp::resource_provider::ResourceProvider;
use crate::mcp::resource_provider_handler::ResourceProviderHandler;
use crate::mcp::resource_provider_list_params::ResourceProviderListParams;
use crate::mcp::resource_reference::ResourceReference;
use crate::mcp::resource_template::ResourceTemplate;

struct FoundProvider<'provider> {
    pub provider: &'provider ResourceProviderHandler,
    pub resource_reference: ResourceReference,
}

pub struct ResourceListAggregate {
    /// Providers need to be sorted for the offset to work
    pub providers: BTreeSet<ResourceProviderHandler>,
}

impl ResourceListAggregate {
    pub async fn list_resources(
        &self,
        ListResourcesParams {
            cursor: ListResourcesCursor { offset },
            per_page,
        }: ListResourcesParams,
    ) -> Result<Vec<Resource>> {
        let mut resources: Vec<Resource> = vec![];
        let mut to_skip = offset;
        let mut to_take = per_page;

        for provider in &self.providers {
            if to_take < 1 {
                break;
            }

            let provider_total = provider.0.total();

            if provider_total < to_skip {
                to_skip -= provider_total;
            } else {
                let provider_available = provider_total - to_skip;
                let provider_offset = to_skip;
                let provider_to_take = cmp::min(provider_available, to_take);

                let mut taken_resources = provider
                    .0
                    .list_resources(ResourceProviderListParams {
                        limit: provider_to_take,
                        offset: provider_offset,
                    })
                    .await?;

                to_skip = 0;
                to_take -= taken_resources.len();

                resources.append(&mut taken_resources);
            }
        }

        Ok(resources)
    }

    pub async fn read_resource_contents(&self, uri: &str) -> Result<Option<ResourceContentParts>> {
        let FoundProvider {
            provider,
            resource_reference,
        } = self.must_get_provider_for_uri(uri)?;

        provider.0.read_resource_contents(resource_reference).await
    }

    pub async fn read_resources_templates_list(&self) -> Result<Vec<ResourceTemplate>> {
        Ok(self
            .providers
            .iter()
            .map(|provider| provider.0.resource_template())
            .collect())
    }

    pub async fn subscribe(&self, uri: &str) -> Result<Option<Receiver<ResourceContentParts>>> {
        let FoundProvider {
            provider,
            resource_reference,
        } = self.must_get_provider_for_uri(uri)?;

        Ok(None)
    }

    fn must_get_provider_for_uri<'provider>(
        &'provider self,
        uri: &str,
    ) -> Result<FoundProvider<'provider>> {
        let parsed_uri: Uri = uri
            .try_into()
            .map_err(|err| anyhow!("{err:#?}"))
            .context(format!("Unable to parse resource URI string: {uri}"))?;

        let resource_reference: ResourceReference = parsed_uri.try_into()?;

        for provider in &self.providers {
            if provider.0.can_handle(&resource_reference) {
                return Ok(FoundProvider {
                    provider: &provider,
                    resource_reference,
                });
            }
        }

        let message = anyhow!("There is no provider that can handle resource: {uri}");

        warn!("{message}");

        Err(message)
    }
}

impl TryFrom<Vec<Arc<dyn ResourceProvider>>> for ResourceListAggregate {
    type Error = anyhow::Error;

    fn try_from(providers: Vec<Arc<dyn ResourceProvider>>) -> Result<Self> {
        Ok(Self {
            providers: providers.into_iter().map(ResourceProviderHandler).collect(),
        })
    }
}

#[cfg(test)]
mod tests {
    use async_trait::async_trait;
    use tokio::sync::mpsc::Receiver;

    use super::*;
    use crate::mcp::resource_provider::ResourceProvider;
    use crate::mcp::resource_template_provider::ResourceTemplateProvider;

    struct TestResourceProvider {
        pub class: String,
        pub total: usize,
    }

    impl ResourceTemplateProvider for TestResourceProvider {
        fn mime_type(&self) -> String {
            "x+foo/bar".to_string()
        }

        fn resource_class(&self) -> String {
            self.class.clone()
        }

        fn resource_scheme(&self) -> String {
            "foo".to_string()
        }
    }

    #[async_trait]
    impl ResourceProvider for TestResourceProvider {
        async fn list_resources(
            &self,
            params: ResourceProviderListParams,
        ) -> Result<Vec<Resource>> {
            let resource_class = self.resource_class();
            let mut resources: Vec<Resource> = Vec::new();

            for i in params.range() {
                resources.push(Resource {
                    description: format!("description_p{resource_class}_r{i}"),
                    name: format!("name_p{resource_class}_r{i}"),
                    title: format!("title_p{resource_class}_r{i}"),
                    uri: format!("uri_p{resource_class}_r{i}"),
                });
            }

            Ok(resources)
        }

        async fn read_resource_contents(
            &self,
            _: ResourceReference,
        ) -> Result<Option<ResourceContentParts>> {
            unimplemented!()
        }

        async fn subscribe(
            &self,
            _: ResourceReference,
        ) -> Result<Option<Receiver<ResourceContentParts>>> {
            unimplemented!()
        }

        fn total(&self) -> usize {
            self.total
        }
    }

    #[tokio::test]
    async fn test_list_resources() -> Result<()> {
        let resource_list_aggregate: ResourceListAggregate = vec![
            Arc::new(TestResourceProvider {
                class: "1".to_string(),
                total: 3,
            }) as Arc<dyn ResourceProvider>,
            Arc::new(TestResourceProvider {
                class: "2".to_string(),
                total: 2,
            }) as Arc<dyn ResourceProvider>,
        ]
        .try_into()?;

        let resources_batch_1 = resource_list_aggregate
            .list_resources(ListResourcesParams {
                cursor: ListResourcesCursor { offset: 0 },
                per_page: 2,
            })
            .await?;

        assert_eq!(resources_batch_1.len(), 2);
        assert_eq!(
            resources_batch_1.first().unwrap().name,
            "name_p1_r0".to_string()
        );
        assert_eq!(
            resources_batch_1.get(1).unwrap().name,
            "name_p1_r1".to_string()
        );

        let resources_batch_2 = resource_list_aggregate
            .list_resources(ListResourcesParams {
                cursor: ListResourcesCursor { offset: 2 },
                per_page: 5,
            })
            .await?;

        assert_eq!(resources_batch_2.len(), 3);
        assert_eq!(
            resources_batch_2.first().unwrap().name,
            "name_p1_r2".to_string()
        );
        assert_eq!(
            resources_batch_2.get(1).unwrap().name,
            "name_p2_r0".to_string()
        );
        assert_eq!(
            resources_batch_2.get(2).unwrap().name,
            "name_p2_r1".to_string()
        );

        Ok(())
    }
}
