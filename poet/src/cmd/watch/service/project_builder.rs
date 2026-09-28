use std::sync::Arc;

use async_trait::async_trait;
use esbuild_metafile::esbuild_metafile::EsbuildMetafile;
use futures_util::TryFutureExt as _;
use log::debug;
use log::info;
use poet_assets::asset_path_renderer::AssetPathRenderer;
use poet_content::build_authors::build_authors;
use poet_content::build_project::build_project;
use poet_content::build_project_params::BuildProjectParams;
use poet_content::build_project_result::BuildProjectResult;
use poet_filesystem::storage::Storage;
use rhai_components::rhai_template_renderer::RhaiTemplateRenderer;
use tokio::sync::Notify;
use tokio_util::sync::CancellationToken;

use crate::cmd::service::Service;
use crate::holder::Holder;
use crate::holder_state::HolderState;
use crate::poet_error::PoetError;
use crate::report_poet_error::report_poet_error;

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
        build_authors(self.source_filesystem.as_ref())
            .map_err(PoetError::BuildAuthors)
            .and_then(|authors| {
                build_project(BuildProjectParams {
                    asset_path_renderer: self.asset_path_renderer.clone(),
                    authors,
                    esbuild_metafile,
                    generate_sitemap: self.generate_sitemap,
                    generated_page_base_path: self.generated_page_base_path.clone(),
                    is_watching: true,
                    rhai_template_renderer,
                    source_filesystem: self.source_filesystem.as_ref(),
                })
                .map_err(PoetError::BuildProject)
            })
            .map_ok(
                |build_project_result_stub| match self.build_project_result_holder.get() {
                    HolderState::Ready(previous_build_project_result) => build_project_result_stub
                        .changed_compared_to(&previous_build_project_result),
                    HolderState::NotReady => build_project_result_stub.into(),
                },
            )
            .await
    }

    async fn rebuild_project(&self) {
        match (
            self.esbuild_metafile_holder.get(),
            self.rhai_template_renderer_holder.get(),
        ) {
            (HolderState::Ready(esbuild_metafile), HolderState::Ready(rhai_template_renderer)) => {
                self.build_project_with(esbuild_metafile, rhai_template_renderer)
                    .await
                    .map_or_else(report_poet_error, |build_project_result| {
                        self.build_project_result_holder.set(build_project_result);

                        info!("Build successful");
                    });
            }
            _ => debug!("Esbuild metafile or shortcodes are not ready yet. Skipping build"),
        }
    }
}

#[async_trait]
impl Service for ProjectBuilder {
    async fn run(&self) -> Result<(), PoetError> {
        let mut esbuild_metafile_updates = self.esbuild_metafile_holder.subscribe();
        let mut rhai_template_renderer_updates = self.rhai_template_renderer_holder.subscribe();

        loop {
            self.rebuild_project().await;

            tokio::select! {
                Ok(()) = esbuild_metafile_updates.changed() => {},
                () = self.on_author_file_changed.notified() => {},
                () = self.on_content_file_changed.notified() => {},
                Ok(()) = rhai_template_renderer_updates.changed() => {},
                () = self.ctrlc_notifier.cancelled() => break,
            }
        }

        Ok(())
    }
}
