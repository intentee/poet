use std::sync::atomic;

use anyhow::Result;
use anyhow::anyhow;
use async_trait::async_trait;
use mime::TEXT_HTML_UTF_8;

use crate::build_project::build_project_result_holder::BuildProjectResultHolder;
use crate::filesystem::Filesystem;
use crate::filesystem::read_file_contents_result::ReadFileContentsResult;
use crate::mcp::jsonrpc::response::success::resources_read::ResourceContent;
use crate::mcp::jsonrpc::response::success::resources_read::TextResourceContent;
use crate::mcp::resource::Resource;
use crate::mcp::resource_provider::ResourceProvider;
use crate::mcp::resource_provider_list_params::ResourceProviderListParams;

#[derive(Clone)]
pub struct McpResourceProviderGeneratedPages(pub BuildProjectResultHolder);

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
        resource_uri: String,
        resource_path: String,
    ) -> Result<Option<Vec<ResourceContent>>> {
        let build_project_result = self.0.must_get_build_project_result().await?;

        match build_project_result
            .markdown_document_reference_collection
            .get(&resource_path)
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
                        mime_type: TEXT_HTML_UTF_8.to_string(),
                        text: contents,
                        uri: resource_uri,
                    })]))
                }
                ReadFileContentsResult::Directory | ReadFileContentsResult::NotFound => Ok(None),
            },
            None => Ok(None),
        }
    }

    fn resource_class(&self) -> String {
        "generated".to_string()
    }

    fn total(&self) -> usize {
        self.0.total.load(atomic::Ordering::Relaxed)
    }
}
