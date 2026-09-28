use std::sync::Arc;

use async_trait::async_trait;
use log::debug;
use poet_assets::asset_path_renderer::AssetPathRenderer;
use poet_content::build_project_result::BuildProjectResult;
use poet_filesystem::storage::Storage;
use poet_prompt::build_prompt_document_controller_collection::build_prompt_document_controller_collection;
use poet_prompt::build_prompt_document_controller_collection_params::BuildPromptDocumentControllerCollectionParams;
use poet_prompt::prompt_document_controller_collection::PromptDocumentControllerCollection;
use poet_prompt::prompt_rendering_context::PromptRenderingContext;
use tokio::sync::Notify;
use tokio_util::sync::CancellationToken;

use crate::cmd::service::Service;
use crate::holder::Holder;
use crate::holder_state::HolderState;
use crate::poet_error::PoetError;
use crate::report_poet_error::report_poet_error;

pub struct PromptDocumentControllerCollectionBuilder {
    pub asset_path_renderer: AssetPathRenderer,
    pub build_project_result_holder: Holder<BuildProjectResult>,
    pub ctrlc_notifier: CancellationToken,
    pub on_prompt_file_changed: Arc<Notify>,
    pub prompt_document_controller_collection_holder:
        Holder<Arc<PromptDocumentControllerCollection>>,
    pub source_filesystem: Arc<Storage>,
}

impl PromptDocumentControllerCollectionBuilder {
    async fn build_prompt_document_controllers(&self) {
        let HolderState::Ready(BuildProjectResult {
            content_document_linker,
            esbuild_metafile,
            rhai_template_renderer,
            ..
        }) = self.build_project_result_holder.get()
        else {
            debug!(
                "Build project result is not ready yet to be used with prompt controllers builder"
            );

            return;
        };

        build_prompt_document_controller_collection(
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
        .map(|prompt_document_controller_collection| {
            self.prompt_document_controller_collection_holder
                .set(Arc::new(prompt_document_controller_collection));
        })
        .map_err(PoetError::BuildPrompts)
        .unwrap_or_else(report_poet_error);
    }
}

#[async_trait]
impl Service for PromptDocumentControllerCollectionBuilder {
    async fn run(&self) -> Result<(), PoetError> {
        let mut build_project_result_updates = self.build_project_result_holder.subscribe();

        loop {
            self.build_prompt_document_controllers().await;

            tokio::select! {
                Ok(()) = build_project_result_updates.changed() => {},
                () = self.on_prompt_file_changed.notified() => {},
                () = self.ctrlc_notifier.cancelled() => break,
            }
        }

        Ok(())
    }
}
