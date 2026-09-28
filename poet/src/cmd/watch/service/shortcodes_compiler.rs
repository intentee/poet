use std::sync::Arc;

use async_trait::async_trait;
use log::error;
use poet_error_chain::error_chain::ErrorChain;
use poet_filesystem::storage::Storage;
use rhai_components::rhai_template_renderer::RhaiTemplateRenderer;
use tokio::sync::Notify;
use tokio_util::sync::CancellationToken;

use crate::cmd::service::Service;
use crate::compile_poet_shortcodes::compile_poet_shortcodes;
use crate::holder::Holder;
use crate::poet_error::PoetError;

pub struct ShortcodesCompiler {
    pub ctrlc_notifier: CancellationToken,
    pub on_shortcode_file_changed: Arc<Notify>,
    pub rhai_template_renderer_holder: Holder<RhaiTemplateRenderer>,
    pub source_filesystem: Arc<Storage>,
}

impl ShortcodesCompiler {
    async fn compile_shortcodes(&self) {
        match compile_poet_shortcodes(&self.source_filesystem).await {
            Ok(rhai_template_renderer) => self
                .rhai_template_renderer_holder
                .set(rhai_template_renderer),
            Err(mdx_error) => error!(
                "{}",
                ErrorChain {
                    error: &PoetError::CompileShortcodes(mdx_error)
                }
            ),
        }
    }
}

#[async_trait]
impl Service for ShortcodesCompiler {
    async fn run(&self) -> Result<(), PoetError> {
        loop {
            self.compile_shortcodes().await;

            tokio::select! {
                () = self.on_shortcode_file_changed.notified() => {},
                () = self.ctrlc_notifier.cancelled() => break,
            }
        }

        Ok(())
    }
}
