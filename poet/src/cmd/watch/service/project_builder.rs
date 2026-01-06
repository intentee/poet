use std::sync::Arc;

use anyhow::Result;
use async_trait::async_trait;
use log::debug;
use log::error;
use log::info;
use tokio::sync::Notify;
use tokio_util::sync::CancellationToken;

use crate::asset_path_renderer::AssetPathRenderer;
use crate::build_project::BuildProjectParams;
use crate::build_project::build_project;
use crate::build_project::build_project_result_holder::BuildProjectResultHolder;
use crate::cmd::service::Service;
use crate::esbuild_metafile_holder::EsbuildMetaFileHolder;
use crate::filesystem::storage::Storage;
use crate::holder::Holder as _;
use crate::mcp::jsonrpc::JSONRPC_VERSION;
use crate::mcp::jsonrpc::notification::resources_list_changed::ResourcesListChanged;
use crate::mcp::jsonrpc::server_to_client_notification::ServerToClientNotification;
use crate::mcp::session_manager::SessionManager;
use crate::rhai_template_renderer_holder::RhaiTemplateRendererHolder;

pub struct ProjectBuilder {
    pub asset_path_renderer: AssetPathRenderer,
    pub build_project_result_holder: BuildProjectResultHolder,
    pub ctrlc_notifier: CancellationToken,
    pub esbuild_metafile_holder: EsbuildMetaFileHolder,
    pub generated_page_base_path: String,
    pub on_author_file_changed: Arc<Notify>,
    pub on_blog_file_changed: Arc<Notify>,
    pub on_content_file_changed: Arc<Notify>,
    pub rhai_template_renderer_holder: RhaiTemplateRendererHolder,
    pub session_manager: SessionManager,
    pub source_filesystem: Arc<Storage>,
}

impl ProjectBuilder {
    async fn do_build_project(&self) {
        let esbuild_metafile = match self.esbuild_metafile_holder.get().await {
            Some(esbuild_metafile) => esbuild_metafile,
            None => {
                debug!("Esbuild metafile is not ready yet. Skipping build");

                return;
            }
        };

        let rhai_template_renderer = match self.rhai_template_renderer_holder.get().await {
            Some(rhai_template_renderer) => rhai_template_renderer,
            None => {
                debug!("Rhai components are not compiled yet. Skipping build");

                return;
            }
        };

        match build_project(BuildProjectParams {
            asset_path_renderer: self.asset_path_renderer.clone(),
            esbuild_metafile,
            generated_page_base_path: self.generated_page_base_path.clone(),
            is_watching: true,
            rhai_template_renderer,
            source_filesystem: self.source_filesystem.clone(),
        })
        .await
        {
            Ok(build_project_result_stub) => {
                self.build_project_result_holder
                    .set(Some(
                        if let Some(old_build_project_result) =
                            self.build_project_result_holder.get().await
                        {
                            build_project_result_stub.changed_compared_to(old_build_project_result)
                        } else {
                            build_project_result_stub.into()
                        },
                    ))
                    .await;

                if let Err(err) = self
                    .session_manager
                    .broadcast(ServerToClientNotification::ResourcesListChanged(
                        ResourcesListChanged {
                            jsonrpc: JSONRPC_VERSION.to_string(),
                        },
                    ))
                    .await
                {
                    error!("Failed to notify MCP sessions: {err:#?}");
                }

                info!("Build successful");
            }
            Err(err) => error!("Failed to build project: {err:#?}"),
        }
    }
}

#[async_trait]
impl Service for ProjectBuilder {
    async fn run(&self) -> Result<()> {
        loop {
            self.do_build_project().await;

            tokio::select! {
                _ = self.esbuild_metafile_holder.update_notifier.notified() => continue,
                _ = self.on_author_file_changed.notified() => continue,
                _ = self.on_blog_file_changed.notified() => continue,
                _ = self.on_content_file_changed.notified() => continue,
                _ = self.rhai_template_renderer_holder.update_notifier.notified() => continue,
                _ = self.ctrlc_notifier.cancelled() => break,
            }
        }

        Ok(())
    }
}
