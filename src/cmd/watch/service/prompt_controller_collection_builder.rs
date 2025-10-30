use std::sync::Arc;

use anyhow::Result;
use async_trait::async_trait;
use log::debug;
use log::error;
use tokio::sync::Notify;
use tokio_util::sync::CancellationToken;

use crate::asset_path_renderer::AssetPathRenderer;
use crate::build_project::build_project_result::BuildProjectResult;
use crate::build_project::build_project_result_holder::BuildProjectResultHolder;
use crate::build_prompt_document_controller_collection::build_prompt_document_controller_collection;
use crate::build_prompt_document_controller_collection::build_prompt_document_controller_collection_params::BuildPromptControllerCollectionParams;
use crate::cmd::service::Service;
use crate::esbuild_metafile_holder::EsbuildMetaFileHolder;
use crate::filesystem::storage::Storage;
use crate::holder::Holder as _;
use crate::prompt_controller_collection_holder::PromptControllerCollectionHolder;
use crate::rhai_template_renderer_holder::RhaiTemplateRendererHolder;

pub struct PromptControllerCollectionBuilder {
    pub asset_path_renderer: AssetPathRenderer,
    pub build_project_result_holder: BuildProjectResultHolder,
    pub ctrlc_notifier: CancellationToken,
    pub esbuild_metafile_holder: EsbuildMetaFileHolder,
    pub on_prompt_file_changed: Arc<Notify>,
    pub prompt_controller_collection_holder: PromptControllerCollectionHolder,
    pub rhai_template_renderer_holder: RhaiTemplateRendererHolder,
    pub source_filesystem: Arc<Storage>,
}

impl PromptControllerCollectionBuilder {
    async fn do_build_prompt_controllers(&self) {
        let content_document_linker = match self.build_project_result_holder.get().await {
            Some(BuildProjectResult {
                content_document_linker,
                ..
            }) => content_document_linker,
            None => {
                debug!(
                    "Build project result is not ready yet to be used with prompt controllers builder"
                );

                return;
            }
        };

        let esbuild_metafile = match self.esbuild_metafile_holder.get().await {
            Some(esbuild_metafile) => esbuild_metafile,
            None => {
                debug!(
                    "Esbuild metafile is not ready yet to be used with prompt controllers builder"
                );

                return;
            }
        };

        let rhai_template_renderer = match self.rhai_template_renderer_holder.get().await {
            Some(rhai_template_renderer) => rhai_template_renderer,
            None => {
                debug!(
                    "Rhai template renderer is not ready yet to be used with prompt controllers builder"
                );

                return;
            }
        };

        match build_prompt_document_controller_collection(BuildPromptControllerCollectionParams {
            asset_path_renderer: self.asset_path_renderer.clone(),
            content_document_linker,
            esbuild_metafile,
            rhai_template_renderer,
            source_filesystem: self.source_filesystem.clone(),
        })
        .await
        {
            Ok(prompt_controller_collection) => {
                self.prompt_controller_collection_holder
                    .set(Some(Arc::new(prompt_controller_collection)))
                    .await;
            }
            Err(err) => error!("Failed to build prompts: {err}"),
        }
    }
}

#[async_trait]
impl Service for PromptControllerCollectionBuilder {
    async fn run(&self) -> Result<()> {
        loop {
            self.do_build_prompt_controllers().await;

            tokio::select! {
                _ = self.build_project_result_holder.update_notifier.notified() => continue,
                _ = self.on_prompt_file_changed.notified() => continue,
                _ = self.rhai_template_renderer_holder.update_notifier.notified() => continue,
                _ = self.ctrlc_notifier.cancelled() => break,
            }
        }

        Ok(())
    }
}
