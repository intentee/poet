use std::sync::atomic;

use anyhow::Result;
use anyhow::anyhow;
use async_trait::async_trait;
use tokio::sync::mpsc;
use tokio::sync::mpsc::Receiver;

use crate::build_project::build_project_result_holder::BuildProjectResultHolder;
use crate::filesystem::Filesystem;
use crate::filesystem::read_file_contents_result::ReadFileContentsResult;
use crate::mcp::jsonrpc::response::success::resources_read::ResourceContent;
use crate::mcp::jsonrpc::response::success::resources_read::TextResourceContent;
use crate::mcp::resource::Resource;
use crate::mcp::resource_content_parts::ResourceContentParts;
use crate::mcp::resource_provider::ResourceProvider;
use crate::mcp::resource_provider_list_params::ResourceProviderListParams;
use crate::mcp::resource_reference::ResourceReference;
use crate::mcp::resource_template_provider::ResourceTemplateProvider;

#[derive(Clone)]
pub struct McpResourceProviderGeneratedPages(pub BuildProjectResultHolder);

impl ResourceTemplateProvider for McpResourceProviderGeneratedPages {
    fn mime_type(&self) -> String {
        mime::TEXT_HTML_UTF_8.to_string()
    }

    fn resource_class(&self) -> String {
        "generated".to_string()
    }

    fn resource_scheme(&self) -> String {
        "poet".to_string()
    }
}

#[async_trait]
impl ResourceProvider for McpResourceProviderGeneratedPages {
    async fn list_resources(
        &self,
        ResourceProviderListParams { limit, offset }: ResourceProviderListParams,
    ) -> Result<Vec<Resource>> {
        Ok(self
            .0
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
        ResourceReference {
            class: _,
            path,
            scheme: _,
            uri,
        }: ResourceReference,
    ) -> Result<Option<ResourceContentParts>> {
        let build_project_result = self.0.must_get_build_project_result().await?;

        match build_project_result
            .markdown_document_reference_collection
            .get(&path)
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
                ReadFileContentsResult::Found { contents } => Ok(Some(
                    ResourceContent::Text(TextResourceContent {
                        meta: None,
                        mime_type: self.mime_type(),
                        text: contents,
                        uri: uri.to_string(),
                    })
                    .into(),
                )),
                ReadFileContentsResult::Directory | ReadFileContentsResult::NotFound => Ok(None),
            },
            None => Ok(None),
        }
    }

    async fn subscribe(
        &self,
        resource_reference: ResourceReference,
    ) -> Result<Option<Receiver<ResourceContentParts>>> {
        let (resource_content_parts_tx, resource_content_parts_rx) = mpsc::channel(3);

        Ok(Some(resource_content_parts_rx))
    }

    fn total(&self) -> usize {
        self.0.total.load(atomic::Ordering::Relaxed)
    }
}
