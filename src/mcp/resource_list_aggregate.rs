use std::cmp;
use std::collections::BTreeSet;
use std::sync::Arc;

use anyhow::Result;

use crate::mcp::list_resources_cursor::ListResourcesCursor;
use crate::mcp::list_resources_params::ListResourcesParams;
use crate::mcp::resource::Resource;
use crate::mcp::resource_provider::ResourceProvider;
use crate::mcp::resource_provider_handler::ResourceProviderHandler;

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
                    .list_resources(provider_offset, provider_to_take)
                    .await?;

                to_skip = 0;
                to_take -= taken_resources.len();

                resources.append(&mut taken_resources);
            }
        }

        Ok(resources)
    }
}

impl From<Vec<Arc<dyn ResourceProvider>>> for ResourceListAggregate {
    fn from(providers: Vec<Arc<dyn ResourceProvider>>) -> Self {
        Self {
            providers: providers.into_iter().map(ResourceProviderHandler).collect(),
        }
    }
}

#[cfg(test)]
mod tests {
    use async_trait::async_trait;

    use super::*;
    use crate::mcp::resource_provider::ResourceProvider;

    struct TestResourceProvider {
        pub id: String,
        pub total: usize,
    }

    #[async_trait]
    impl ResourceProvider for TestResourceProvider {
        fn id(&self) -> String {
            self.id.clone()
        }

        async fn list_resources(&self, offset: usize, limit: usize) -> Result<Vec<Resource>> {
            let id = self.id();
            let mut resources: Vec<Resource> = Vec::new();

            for i in offset..(offset + limit) {
                resources.push(Resource {
                    description: format!("description_p{id}_r{i}"),
                    name: format!("name_p{id}_r{i}"),
                    title: format!("title_p{id}_r{i}"),
                });
            }

            Ok(resources)
        }

        fn total(&self) -> usize {
            self.total
        }
    }

    #[tokio::test]
    async fn test_list_resources() -> Result<()> {
        let resource_list_aggregate: ResourceListAggregate = vec![
            Arc::new(TestResourceProvider {
                id: "1".to_string(),
                total: 3,
            }) as Arc<dyn ResourceProvider>,
            Arc::new(TestResourceProvider {
                id: "2".to_string(),
                total: 2,
            }) as Arc<dyn ResourceProvider>,
        ]
        .into();

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
