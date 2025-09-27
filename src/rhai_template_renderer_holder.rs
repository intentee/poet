use std::sync::Arc;

use async_trait::async_trait;
use tokio::sync::Notify;
use tokio::sync::RwLock;

use crate::holder::Holder;
use crate::rhai_template_renderer::RhaiTemplateRenderer;

#[derive(Clone, Default)]
pub struct RhaiTemplateRendererHolder {
    rhai_template_renderer: Arc<RwLock<Option<RhaiTemplateRenderer>>>,
    pub update_notifier: Arc<Notify>,
}

#[async_trait]
impl Holder for RhaiTemplateRendererHolder {
    type Item = RhaiTemplateRenderer;

    fn rw_lock(&self) -> Arc<RwLock<Option<Self::Item>>> {
        self.rhai_template_renderer.clone()
    }

    fn update_notifier(&self) -> Arc<Notify> {
        self.update_notifier.clone()
    }
}
