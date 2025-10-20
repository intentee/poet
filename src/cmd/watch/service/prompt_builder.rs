use std::sync::Arc;

use anyhow::Result;
use async_trait::async_trait;
use log::error;
use tokio::sync::Notify;
use tokio_util::sync::CancellationToken;

use crate::build_prompt_controller_collection::build_prompt_controller_collection;
use crate::build_prompt_controller_collection::build_prompt_controller_collection_params::BuildPromptControllerCollectionParams;
use crate::cmd::watch::service::Service;
use crate::filesystem::storage::Storage;

pub struct PromptBuilder {
    pub ctrlc_notifier: CancellationToken,
    pub on_prompt_file_changed: Arc<Notify>,
    pub source_filesystem: Arc<Storage>,
}

impl PromptBuilder {
    async fn do_build_prompt_controllers(&self) {
        match build_prompt_controller_collection(BuildPromptControllerCollectionParams {
            source_filesystem: self.source_filesystem.clone(),
        })
        .await
        {
            Ok(build_prompt_controllers_result) => {}
            Err(err) => error!("Failed to build prompts: {err}"),
        }
    }
}

#[async_trait]
impl Service for PromptBuilder {
    async fn run(&self) -> Result<()> {
        loop {
            self.do_build_prompt_controllers().await;

            tokio::select! {
                _ = self.on_prompt_file_changed.notified() => continue,
                _ = self.ctrlc_notifier.cancelled() => break,
            }
        }

        Ok(())
    }
}
