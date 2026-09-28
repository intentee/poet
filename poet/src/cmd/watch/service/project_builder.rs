use std::sync::Arc;

use async_trait::async_trait;
use esbuild_metafile::esbuild_metafile::EsbuildMetafile;
use log::debug;
use log::error;
use log::info;
use poet_assets::asset_path_renderer::AssetPathRenderer;
use poet_content::build_authors::build_authors;
use poet_content::build_project::build_project;
use poet_content::build_project_params::BuildProjectParams;
use poet_content::build_project_result::BuildProjectResult;
use poet_error_chain::error_chain::ErrorChain;
use poet_filesystem::storage::Storage;
use rhai_components::rhai_template_renderer::RhaiTemplateRenderer;
use tokio::sync::Notify;
use tokio_util::sync::CancellationToken;

use crate::cmd::service::Service;
use crate::holder::Holder;
use crate::holder_state::HolderState;
use crate::poet_error::PoetError;

pub struct ProjectBuilder {
    pub asset_path_renderer: AssetPathRenderer,
    pub build_project_result_holder: Holder<BuildProjectResult>,
    pub ctrlc_notifier: CancellationToken,
    pub esbuild_metafile_holder: Holder<Arc<EsbuildMetafile>>,
    pub generate_sitemap: bool,
    pub generated_page_base_path: String,
    pub on_author_file_changed: Arc<Notify>,
    pub on_content_file_changed: Arc<Notify>,
    pub rhai_template_renderer_holder: Holder<RhaiTemplateRenderer>,
    pub source_filesystem: Arc<Storage>,
}

impl ProjectBuilder {
    async fn build_project_with(
        &self,
        esbuild_metafile: Arc<EsbuildMetafile>,
        rhai_template_renderer: RhaiTemplateRenderer,
    ) -> Result<BuildProjectResult, PoetError> {
        let authors = build_authors(self.source_filesystem.as_ref())
            .await
            .map_err(PoetError::BuildAuthors)?;
        let build_project_result_stub = build_project(BuildProjectParams {
            asset_path_renderer: self.asset_path_renderer.clone(),
            authors,
            esbuild_metafile,
            generate_sitemap: self.generate_sitemap,
            generated_page_base_path: self.generated_page_base_path.clone(),
            is_watching: true,
            rhai_template_renderer,
            source_filesystem: self.source_filesystem.as_ref(),
        })
        .await
        .map_err(PoetError::BuildProject)?;

        Ok(match self.build_project_result_holder.get() {
            HolderState::Ready(previous_build_project_result) => {
                build_project_result_stub.changed_compared_to(&previous_build_project_result)
            }
            HolderState::NotReady => build_project_result_stub.into(),
        })
    }

    async fn rebuild_project(&self) {
        let HolderState::Ready(esbuild_metafile) = self.esbuild_metafile_holder.get() else {
            debug!("Esbuild metafile is not ready yet. Skipping build");

            return;
        };
        let HolderState::Ready(rhai_template_renderer) = self.rhai_template_renderer_holder.get()
        else {
            debug!("Rhai components are not compiled yet. Skipping build");

            return;
        };

        match self
            .build_project_with(esbuild_metafile, rhai_template_renderer)
            .await
        {
            Ok(build_project_result) => {
                self.build_project_result_holder.set(build_project_result);

                info!("Build successful");
            }
            Err(poet_error) => error!("{}", ErrorChain { error: &poet_error }),
        }
    }
}

#[async_trait]
impl Service for ProjectBuilder {
    async fn run(&self) -> Result<(), PoetError> {
        loop {
            self.rebuild_project().await;

            tokio::select! {
                () = self.esbuild_metafile_holder.update_notifier.notified() => {},
                () = self.on_author_file_changed.notified() => {},
                () = self.on_content_file_changed.notified() => {},
                () = self.rhai_template_renderer_holder.update_notifier.notified() => {},
                () = self.ctrlc_notifier.cancelled() => break,
            }
        }

        Ok(())
    }
}
