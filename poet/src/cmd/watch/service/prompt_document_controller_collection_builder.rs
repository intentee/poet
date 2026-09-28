use std::sync::Arc;

use async_trait::async_trait;
use esbuild_metafile::esbuild_metafile::EsbuildMetafile;
use log::debug;
use log::error;
use poet_assets::asset_path_renderer::AssetPathRenderer;
use poet_content::build_project_result::BuildProjectResult;
use poet_error_chain::error_chain::ErrorChain;
use poet_filesystem::storage::Storage;
use poet_prompt::build_prompt_document_controller_collection::build_prompt_document_controller_collection;
use poet_prompt::build_prompt_document_controller_collection_params::BuildPromptDocumentControllerCollectionParams;
use poet_prompt::prompt_document_controller_collection::PromptDocumentControllerCollection;
use poet_prompt::prompt_rendering_context::PromptRenderingContext;
use rhai_components::rhai_template_renderer::RhaiTemplateRenderer;
use tokio::sync::Notify;
use tokio_util::sync::CancellationToken;

use crate::cmd::service::Service;
use crate::holder::Holder;
use crate::holder_state::HolderState;
use crate::poet_error::PoetError;

pub struct PromptDocumentControllerCollectionBuilder {
    pub asset_path_renderer: AssetPathRenderer,
    pub build_project_result_holder: Holder<BuildProjectResult>,
    pub ctrlc_notifier: CancellationToken,
    pub esbuild_metafile_holder: Holder<Arc<EsbuildMetafile>>,
    pub on_prompt_file_changed: Arc<Notify>,
    pub prompt_document_controller_collection_holder:
        Holder<Arc<PromptDocumentControllerCollection>>,
    pub rhai_template_renderer_holder: Holder<RhaiTemplateRenderer>,
    pub source_filesystem: Arc<Storage>,
}

impl PromptDocumentControllerCollectionBuilder {
    async fn build_prompt_document_controllers(&self) {
        let HolderState::Ready(BuildProjectResult {
            content_document_linker,
            ..
        }) = self.build_project_result_holder.get()
        else {
            debug!(
                "Build project result is not ready yet to be used with prompt controllers builder"
            );

            return;
        };
        let HolderState::Ready(esbuild_metafile) = self.esbuild_metafile_holder.get() else {
            debug!("Esbuild metafile is not ready yet to be used with prompt controllers builder");

            return;
        };
        let HolderState::Ready(rhai_template_renderer) = self.rhai_template_renderer_holder.get()
        else {
            debug!(
                "Rhai template renderer is not ready yet to be used with prompt controllers builder"
            );

            return;
        };

        match build_prompt_document_controller_collection(
            BuildPromptDocumentControllerCollectionParams {
                rendering_context: PromptRenderingContext {
                    asset_path_renderer: self.asset_path_renderer.clone(),
                    content_document_linker,
                    esbuild_metafile,
                    rhai_template_renderer,
                },
                source_filesystem: self.source_filesystem.as_ref(),
            },
        )
        .await
        {
            Ok(prompt_document_controller_collection) => self
                .prompt_document_controller_collection_holder
                .set(Arc::new(prompt_document_controller_collection)),
            Err(prompt_error) => error!(
                "{}",
                ErrorChain {
                    error: &PoetError::BuildPrompts(prompt_error)
                }
            ),
        }
    }
}

#[async_trait]
impl Service for PromptDocumentControllerCollectionBuilder {
    async fn run(&self) -> Result<(), PoetError> {
        loop {
            self.build_prompt_document_controllers().await;

            tokio::select! {
                () = self.build_project_result_holder.update_notifier.notified() => {},
                () = self.on_prompt_file_changed.notified() => {},
                () = self.rhai_template_renderer_holder.update_notifier.notified() => {},
                () = self.ctrlc_notifier.cancelled() => break,
            }
        }

        Ok(())
    }
}
