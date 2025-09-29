use std::sync::Arc;
use std::sync::atomic;
use std::sync::atomic::AtomicUsize;

use anyhow::Result;
use anyhow::anyhow;
use async_trait::async_trait;
use http::Uri;
use tokio::sync::Notify;
use tokio::sync::RwLock;

use crate::build_project::build_project_result::BuildProjectResult;
use crate::filesystem::Filesystem;
use crate::filesystem::read_file_contents_result::ReadFileContentsResult;
use crate::holder::Holder;
use crate::mcp::jsonrpc::response::success::resources_read::ResourceContent;
use crate::mcp::jsonrpc::response::success::resources_read::TextResourceContent;
use crate::mcp::resource::Resource;
use crate::mcp::resource_provider::ResourceProvider;
use crate::mcp::resource_provider_list_params::ResourceProviderListParams;

#[derive(Clone, Default)]
pub struct BuildProjectResultHolder {
    build_project_result_lock: Arc<RwLock<Option<BuildProjectResult>>>,
    total: Arc<AtomicUsize>,
    pub update_notifier: Arc<Notify>,
}

impl BuildProjectResultHolder {
    async fn must_get_build_project_result(&self) -> Result<BuildProjectResult> {
        self.get().await.ok_or_else(|| {
            anyhow!("Server is still starting up, or there are no successful builds yet")
        })
    }
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
        self.build_project_result_lock.clone()
    }

    fn update_notifier(&self) -> Arc<Notify> {
        self.update_notifier.clone()
    }
}

#[async_trait]
impl ResourceProvider for BuildProjectResultHolder {
    async fn list_resources(
        &self,
        ResourceProviderListParams { limit, offset }: ResourceProviderListParams,
    ) -> Result<Vec<Resource>> {
        Ok(self
            .must_get_build_project_result()
            .await?
            .markdown_document_reference_collection
            .iter()
            .skip(offset)
            .take(limit)
            .map(|(basename, reference)| Resource {
                description: reference.front_matter.description.to_owned(),
                name: basename.to_string(),
                title: reference.front_matter.title.to_owned(),
                uri: self.resource_uri(basename),
            })
            .collect())
    }

    async fn read_resource_contents(
        &self,
        resource_uri: Uri,
    ) -> Result<Option<Vec<ResourceContent>>> {
        let resource_path = resource_uri.path();
        let basename = if resource_path.starts_with("/") {
            resource_path
                .strip_prefix("/")
                .ok_or_else(|| anyhow!("Unable to strip resource path prefix"))?
        } else {
            resource_path
        };

        let build_project_result = self.must_get_build_project_result().await?;

        match build_project_result
            .markdown_document_reference_collection
            .get(basename)
        {
            Some(reference) => match build_project_result
                .memory_filesystem
                .read_file_contents(
                    &reference
                        .target_file_relative_path()
                        .map_err(|message| anyhow!("{message}"))?,
                )
                .await?
            {
                ReadFileContentsResult::Found { contents } => {
                    Ok(Some(vec![ResourceContent::Text(TextResourceContent {
                        meta: None,
                        mime_type: mime::TEXT_HTML_UTF_8.to_string(),
                        text: contents,
                        uri: resource_uri.to_string(),
                    })]))
                }
                ReadFileContentsResult::Directory | ReadFileContentsResult::NotFound => Ok(None),
            },
            None => Ok(None),
        }
    }

    fn resource_class(&self) -> String {
        "content".to_string()
    }

    fn total(&self) -> usize {
        self.total.load(atomic::Ordering::Relaxed)
    }
}
