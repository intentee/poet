use std::sync::Arc;
use std::sync::atomic;
use std::sync::atomic::AtomicUsize;

use anyhow::Result;
use anyhow::anyhow;
use async_trait::async_trait;
use tokio::sync::Notify;
use tokio::sync::RwLock;

use crate::build_project::build_project_result::BuildProjectResult;
use crate::holder::Holder;
use crate::mcp::resource::Resource;
use crate::mcp::resource_provider::ResourceProvider;
use crate::mcp::resource_provider_list_params::ResourceProviderListParams;

#[derive(Clone, Default)]
pub struct BuildProjectResultHolder {
    rhai_template_renderer: Arc<RwLock<Option<BuildProjectResult>>>,
    total: Arc<AtomicUsize>,
    pub update_notifier: Arc<Notify>,
}

#[async_trait]
impl Holder for BuildProjectResultHolder {
    type Item = BuildProjectResult;

    fn on_update(&self, build_project_result: &Option<Self::Item>) {
        self.total.store(
            if let Some(build_project_result) = build_project_result {
                build_project_result
                    .markdown_document_reference_collection
                    .len()
            } else {
                0
            },
            atomic::Ordering::Relaxed,
        );
    }

    fn rw_lock(&self) -> Arc<RwLock<Option<Self::Item>>> {
        self.rhai_template_renderer.clone()
    }

    fn update_notifier(&self) -> Arc<Notify> {
        self.update_notifier.clone()
    }
}

#[async_trait]
impl ResourceProvider for BuildProjectResultHolder {
    fn id(&self) -> String {
        "build_project_result_holder".to_string()
    }

    async fn list_resources(
        &self,
        ResourceProviderListParams { limit, offset }: ResourceProviderListParams,
    ) -> Result<Vec<Resource>> {
        let build_project_result: BuildProjectResult = self.get().await.ok_or_else(|| {
            anyhow!("Server is still starting up, or there are no successful builds yet")
        })?;

        Ok(build_project_result
            .markdown_document_reference_collection
            .iter()
            .skip(offset)
            .take(limit)
            .map(|(basename, reference)| Resource {
                description: reference.front_matter.description.to_owned(),
                name: basename.to_string(),
                title: reference.front_matter.title.to_owned(),
                uri: "heh".to_string(),
            })
            .collect())
    }

    fn total(&self) -> usize {
        self.total.load(atomic::Ordering::Relaxed)
    }
}
