use std::cmp;
use std::collections::BTreeMap;
use std::sync::Arc;

use anyhow::Result;
use anyhow::anyhow;
use http::Uri;

use crate::mcp::jsonrpc::response::success::resources_read::ResourceContent;
use crate::mcp::list_resources_cursor::ListResourcesCursor;
use crate::mcp::list_resources_params::ListResourcesParams;
use crate::mcp::resource::Resource;
use crate::mcp::resource_provider::ResourceProvider;
use crate::mcp::resource_provider_handler::ResourceProviderHandler;
use crate::mcp::resource_provider_list_params::ResourceProviderListParams;

pub struct ResourceListAggregate {
    /// Providers need to be sorted for the offset to work
    pub providers: BTreeMap<String, ResourceProviderHandler>,
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

        for provider in self.providers.values() {
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

    pub async fn read_resource_contents(&self, uri: &str) -> Result<Option<Vec<ResourceContent>>> {
        let parsed_uri: Uri = uri.try_into()?;
        let resource_class: &str = parsed_uri
            .scheme_str()
            .ok_or_else(|| anyhow!("Resource URI has no scheme"))?;

        let resource_path: String = uri
            .strip_prefix(&format!("{resource_class}://"))
            .ok_or_else(|| anyhow!("Unable to strip resource prefix"))?
            .to_string();

        self.providers
            .get(resource_class)
            .ok_or_else(|| anyhow!("No provider found for resource class: {resource_class}"))?
            .0
            .read_resource_contents(uri.to_string(), resource_path)
            .await
    }
}

impl TryFrom<Vec<Arc<dyn ResourceProvider>>> for ResourceListAggregate {
    type Error = anyhow::Error;

    fn try_from(providers: Vec<Arc<dyn ResourceProvider>>) -> Result<Self> {
        let mut providers_map = BTreeMap::new();

        for provider in providers {
            let resource_class = provider.resource_class();

            if providers_map.contains_key(&resource_class) {
                return Err(anyhow!(
                    "Duplicate resource class provider: {resource_class}"
                ));
            }

            providers_map.insert(resource_class, ResourceProviderHandler(provider));
        }

        Ok(Self {
            providers: providers_map,
        })
    }
}

#[cfg(test)]
mod tests {
    use async_trait::async_trait;

    use super::*;
    use crate::mcp::resource_provider::ResourceProvider;

    struct TestResourceProvider {
        pub class: String,
        pub total: usize,
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
            _: String,
            _: String,
        ) -> Result<Option<Vec<ResourceContent>>> {
            unimplemented!()
        }

        fn resource_class(&self) -> String {
            self.class.clone()
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
