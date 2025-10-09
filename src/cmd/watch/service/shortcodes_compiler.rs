use std::sync::Arc;

use anyhow::Result;
use async_trait::async_trait;
use log::error;
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

impl ShortcodesCompiler {
    async fn do_compile_shortcodes(&self) {
        match compile_shortcodes(self.source_filesystem.clone()).await {
            Ok(rhai_template_renderer) => {
                self.rhai_template_renderer_holder
                    .set(Some(rhai_template_renderer))
                    .await;
            }
            Err(err) => error!("Unable to compile shortcodes: {err:#?}"),
        };
    }
}

#[async_trait]
impl Service for ShortcodesCompiler {
    async fn run(&self) -> Result<()> {
        loop {
            tokio::select! {
                _ = self.on_shortcode_file_changed.notified() => self.do_compile_shortcodes().await,
                _ = self.ctrlc_notifier.cancelled() => {
                    break;
                },
            }
        }

        Ok(())
    }
}
