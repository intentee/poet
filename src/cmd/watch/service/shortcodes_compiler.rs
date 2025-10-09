use std::sync::Arc;

use anyhow::Result;
use async_trait::async_trait;
use tokio::sync::Notify;
use tokio_util::sync::CancellationToken;

use crate::cmd::watch::service::Service;
use crate::compile_shortcodes::compile_shortcodes;
use crate::filesystem::storage::Storage;
use crate::holder::Holder as _;
use crate::rhai_template_renderer_holder::RhaiTemplateRendererHolder;

pub struct ShortcodesCompiler {
    pub ctrlc_notifier: CancellationToken,
    pub on_shortcode_file_changed: Arc<Notify>,
    pub rhai_template_renderer_holder: RhaiTemplateRendererHolder,
    pub source_filesystem: Arc<Storage>,
}

#[async_trait]
impl Service for ShortcodesCompiler {
    async fn run(self: Arc<Self>) -> Result<()> {
        loop {
            let rhai_template_renderer = compile_shortcodes(self.source_filesystem.clone()).await?;

            self.rhai_template_renderer_holder
                .set(Some(rhai_template_renderer))
                .await;

            tokio::select! {
                _ = self.on_shortcode_file_changed.notified() => {},
                _ = self.ctrlc_notifier.cancelled() => {
                    break;
                },
            }
        }

        Ok(())
    }
}
