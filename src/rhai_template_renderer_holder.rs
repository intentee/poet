use std::sync::Arc;

use tokio::sync::Notify;
use tokio::sync::RwLock;

use crate::rhai_template_renderer::RhaiTemplateRenderer;

#[derive(Clone, Default)]
pub struct RhaiTemplateRendererHolder {
    rhai_template_renderer: Arc<RwLock<Option<RhaiTemplateRenderer>>>,
    pub update_notifier: Arc<Notify>,
}

impl RhaiTemplateRendererHolder {
    pub async fn get_rhai_template_renderer(&self) -> Option<RhaiTemplateRenderer> {
        let rhai_template_renderer_opt = self.rhai_template_renderer.read().await;

        rhai_template_renderer_opt.clone()
    }

    pub async fn set_rhai_template_renderer(
        &self,
        rhai_template_renderer: Option<RhaiTemplateRenderer>,
    ) {
        {
            let mut rhai_template_renderer_shared_writer =
                self.rhai_template_renderer.write().await;

            *rhai_template_renderer_shared_writer = rhai_template_renderer;
        }

        self.update_notifier.notify_waiters();
    }
}
