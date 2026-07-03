use std::sync::Arc;
use std::sync::atomic;

use actix_web::rt;
use anyhow::Result;
use async_trait::async_trait;
use tokio::sync::Notify;
use tokio_util::sync::CancellationToken;

use crate::build_project::build_project_result_holder::BuildProjectResultHolder;
use crate::content_document_basename::ContentDocumentBasename;
use crate::holder::Holder as _;
use crate::mcp::resource::Resource;
use crate::mcp::resource_content::ResourceContent;
use crate::mcp::resource_content::TextResourceContent;
use crate::mcp::resource_content_parts::ResourceContentParts;
use crate::mcp::resource_provider::ResourceProvider;
use crate::mcp::resource_provider_list_params::ResourceProviderListParams;
use crate::mcp::resource_reference::ResourceReference;
use crate::mcp::resource_template_provider::ResourceTemplateProvider;

#[derive(Clone, Default)]
pub struct McpResourceProviderContentDocuments(pub BuildProjectResultHolder);

impl ResourceTemplateProvider for McpResourceProviderContentDocuments {
    fn mime_type(&self) -> String {
        "text/markdown".to_string()
    }

    fn resource_class(&self) -> String {
        "content".to_string()
    }

    fn resource_scheme(&self) -> String {
        "poet".to_string()
    }
}

#[async_trait]
impl ResourceProvider for McpResourceProviderContentDocuments {
    async fn list_resources(
        &self,
        ResourceProviderListParams { limit, offset }: ResourceProviderListParams,
    ) -> Result<Vec<Resource>> {
        Ok(self
            .0
            .must_get_build_project_result()
            .await?
            .content_document_sources
            .values()
            .skip(offset)
            .take(limit)
            .map(|content_document_source| {
                let basename_string: String =
                    content_document_source.reference.basename().to_string();

                Resource {
                    description: content_document_source
                        .reference
                        .front_matter
                        .description
                        .to_owned(),
                    title: content_document_source
                        .reference
                        .front_matter
                        .title
                        .to_owned(),
                    uri: self.resource_uri(&basename_string),
                    name: basename_string,
                }
            })
            .collect())
    }

    async fn read_resource_contents(
        &self,
        ResourceReference {
            path, uri_string, ..
        }: ResourceReference,
    ) -> Result<Option<ResourceContentParts>> {
        let basename: ContentDocumentBasename = path.into();
        let build_project_result = self.0.must_get_build_project_result().await?;

        match build_project_result.content_document_sources.get(&basename) {
            Some(content_document_source) => Ok(Some(ResourceContentParts {
                parts: vec![ResourceContent::Text(TextResourceContent {
                    mime_type: self.mime_type(),
                    text: content_document_source.file_entry.contents.clone(),
                    uri: uri_string.clone(),
                })],
                title: content_document_source.reference.front_matter.title.clone(),
                uri: uri_string.clone(),
            })),
            None => Ok(None),
        }
    }

    async fn resource_update_notifier(
        self: Arc<Self>,
        cancellation_token: CancellationToken,
        resource_reference: ResourceReference,
    ) -> Result<Option<Arc<Notify>>> {
        let build_project_result_holder = self.0.clone();
        let build_update_notifier = self.0.update_notifier.clone();
        let resource_update_notifier: Arc<Notify> = Default::default();

        let resource_update_notifier_clone = resource_update_notifier.clone();
        let this = self.clone();

        rt::spawn(async move {
            loop {
                tokio::select! {
                    _ = cancellation_token.cancelled() => break,
                    _ = build_update_notifier.notified() => {
                        if let Some(build_project_result) = build_project_result_holder.get().await {
                            for content_document_source in build_project_result.changed_since_last_build {
                                let reference_uri = this.resource_uri(&content_document_source.relative_path);

                                if reference_uri == resource_reference.uri_string {
                                    resource_update_notifier_clone.notify_waiters();
                                }
                            }
                        }
                    }
                }
            }
        });

        Ok(Some(resource_update_notifier))
    }

    fn total(&self) -> usize {
        self.0.total.load(atomic::Ordering::Relaxed)
    }
}

#[cfg(test)]
mod tests {
    use std::path::Path;

    use tempfile::tempdir;

    use super::*;
    use crate::asset_path_renderer::AssetPathRenderer;
    use crate::build_authors::build_authors;
    use crate::build_project::build_project;
    use crate::build_project::build_project_params::BuildProjectParams;
    use crate::build_project::build_project_result::BuildProjectResult;
    use crate::compile_shortcodes::compile_shortcodes;
    use crate::filesystem::Filesystem as _;
    use crate::filesystem::storage::Storage;
    use crate::mcp::resource_provider_list_params::ResourceProviderListParams;

    async fn build_result() -> Result<BuildProjectResult> {
        let directory = tempdir()?;
        let source_filesystem = Arc::new(Storage {
            base_directory: directory.path().to_path_buf(),
        });

        source_filesystem
            .set_file_contents(
                Path::new("shortcodes/Layout.rhai"),
                "fn template(context, props, content) { component { <html>{content}</html> } }",
            )
            .await?;
        source_filesystem
            .set_file_contents(
                Path::new("content/guide.md"),
                "+++\ndescription = \"Guide description\"\nlayout = \"Layout\"\ntitle = \"Guide\"\n+++\n\nbody\n",
            )
            .await?;

        let rhai_template_renderer = compile_shortcodes(source_filesystem.clone()).await?;
        let authors = build_authors(source_filesystem.clone()).await?;

        Ok(build_project(BuildProjectParams {
            asset_path_renderer: AssetPathRenderer {
                base_path: "/".to_string(),
            },
            authors,
            esbuild_metafile: Default::default(),
            generated_page_base_path: "/".to_string(),
            generate_sitemap: false,
            is_watching: false,
            rhai_template_renderer,
            source_filesystem,
        })
        .await?
        .into())
    }

    fn reference(path: &str) -> ResourceReference {
        ResourceReference {
            class: "content".to_string(),
            path: path.to_string(),
            scheme: "poet".to_string(),
            uri_string: format!("poet://content/{path}"),
        }
    }

    #[tokio::test]
    async fn lists_content_documents_as_resources() -> Result<()> {
        let provider = McpResourceProviderContentDocuments::default();

        provider.0.set(Some(build_result().await?)).await;

        assert_eq!(provider.total(), 1);

        let resources = provider
            .list_resources(ResourceProviderListParams {
                limit: 10,
                offset: 0,
            })
            .await?;

        assert_eq!(resources.len(), 1);
        assert_eq!(resources[0].name, "guide");
        assert_eq!(resources[0].title, "Guide");

        Ok(())
    }

    #[tokio::test]
    async fn reads_existing_document_and_misses_unknown_one() -> Result<()> {
        let provider = McpResourceProviderContentDocuments::default();

        provider.0.set(Some(build_result().await?)).await;

        assert!(
            provider
                .read_resource_contents(reference("guide"))
                .await?
                .is_some()
        );
        assert!(
            provider
                .read_resource_contents(reference("missing"))
                .await?
                .is_none()
        );

        Ok(())
    }
}
