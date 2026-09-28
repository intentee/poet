use std::sync::Arc;
use std::sync::atomic;

use actix_web::rt;
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

use crate::build_project::build_project_result::BuildProjectResult;
use crate::build_project_result_holder::BuildProjectResultHolder;
use crate::content_document_basename::ContentDocumentBasename;
use crate::holder::Holder as _;

#[derive(Clone, Default)]
pub struct McpResourceProviderContentDocuments {
    pub build_project_result_holder: BuildProjectResultHolder,
}

impl McpResourceProviderContentDocuments {
    fn is_updated_by(
        resource_reference: &ResourceReference,
        build_project_result: &BuildProjectResult,
    ) -> bool {
        let subscribed_basename: ContentDocumentBasename = resource_reference.path.clone().into();

        build_project_result
            .changed_since_last_build
            .iter()
            .any(|content_document_source| {
                content_document_source.reference.basename() == subscribed_basename
            })
    }
}

impl ResourceTemplateProvider for McpResourceProviderContentDocuments {
    fn mime_type(&self) -> String {
        "text/markdown".to_owned()
    }

    fn resource_class(&self) -> String {
        "content".to_owned()
    }

    fn resource_scheme(&self) -> String {
        "poet".to_owned()
    }
}

#[async_trait]
impl ResourceProvider for McpResourceProviderContentDocuments {
    async fn list_resources(
        &self,
        ResourceProviderListParams { limit, offset }: ResourceProviderListParams,
    ) -> Result<Vec<Resource>, ProviderError> {
        Ok(self
            .build_project_result_holder
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
                        .clone(),
                    title: content_document_source.reference.front_matter.title.clone(),
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
    ) -> Result<Option<Vec<ResourceContent>>, ProviderError> {
        let basename: ContentDocumentBasename = path.into();
        let build_project_result = self
            .build_project_result_holder
            .must_get_build_project_result()
            .await?;

        Ok(build_project_result
            .content_document_sources
            .get(&basename)
            .map(|content_document_source| {
                vec![ResourceContent::Text(TextResourceContent {
                    mime_type: self.mime_type(),
                    text: content_document_source.file_entry.contents.clone(),
                    uri: uri_string,
                })]
            }))
    }

    fn resource_update_notifier(
        self: Arc<Self>,
        cancellation_token: CancellationToken,
        resource_reference: ResourceReference,
    ) -> Arc<Notify> {
        let build_project_result_holder = self.build_project_result_holder.clone();
        let build_update_notifier = self.build_project_result_holder.update_notifier.clone();
        let resource_update_notifier: Arc<Notify> = Arc::default();
        let notified_resource_update_notifier = resource_update_notifier.clone();

        rt::spawn(async move {
            loop {
                tokio::select! {
                    () = cancellation_token.cancelled() => break,
                    () = build_update_notifier.notified() => {
                        if let Some(build_project_result) = build_project_result_holder.get().await
                            && Self::is_updated_by(&resource_reference, &build_project_result)
                        {
                            notified_resource_update_notifier.notify_waiters();
                        }
                    }
                }
            }
        });

        resource_update_notifier
    }

    fn total(&self) -> usize {
        self.build_project_result_holder
            .total
            .load(atomic::Ordering::Relaxed)
    }
}

#[cfg(test)]
mod tests {
    use std::path::Path;
    use std::sync::Arc;

    use anyhow::Result;
    use poet_assets::asset_path_renderer::AssetPathRenderer;
    use poet_filesystem::filesystem::Filesystem as _;
    use poet_filesystem::storage::Storage;
    use poet_mcp::provider_error::ProviderError;
    use poet_mcp::resource_provider::ResourceProvider as _;
    use poet_mcp::resource_provider_list_params::ResourceProviderListParams;
    use poet_mcp::resource_reference::ResourceReference;
    use tempfile::tempdir;

    use crate::build_authors::build_authors;
    use crate::build_project::build_project;
    use crate::build_project::build_project_params::BuildProjectParams;
    use crate::build_project::build_project_result::BuildProjectResult;
    use crate::build_project::build_project_result_stub::BuildProjectResultStub;
    use crate::compile_poet_shortcodes::compile_poet_shortcodes;
    use crate::holder::Holder as _;
    use crate::mcp_resource_provider_content_documents::McpResourceProviderContentDocuments;

    async fn build_stub(body: &str) -> Result<BuildProjectResultStub> {
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
                &format!(
                    "+++\ndescription = \"Guide description\"\nlayout = \"Layout\"\ntitle = \"Guide\"\n+++\n\n{body}\n"
                ),
            )
            .await?;

        let rhai_template_renderer = compile_poet_shortcodes(&source_filesystem).await?;
        let authors = build_authors(source_filesystem.clone()).await?;

        build_project(BuildProjectParams {
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
        .await
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
    async fn lists_content_documents_as_resources() -> Result<(), ProviderError> {
        let provider = McpResourceProviderContentDocuments::default();

        provider
            .build_project_result_holder
            .set(Some(build_stub("body").await?.into()))
            .await;

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
    async fn reads_existing_document_and_misses_unknown_one() -> Result<(), ProviderError> {
        let provider = McpResourceProviderContentDocuments::default();

        provider
            .build_project_result_holder
            .set(Some(build_stub("body").await?.into()))
            .await;

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

    #[tokio::test]
    async fn changed_subscribed_document_updates_its_resource() -> Result<(), ProviderError> {
        let previous_build: BuildProjectResult = build_stub("body").await?.into();
        let changed_build = build_stub("changed body")
            .await?
            .changed_compared_to(previous_build);

        assert!(McpResourceProviderContentDocuments::is_updated_by(
            &reference("guide"),
            &changed_build
        ));

        Ok(())
    }
}
