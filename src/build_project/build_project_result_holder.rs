use std::sync::Arc;

use async_trait::async_trait;
use tokio::sync::Notify;
use tokio::sync::RwLock;

use crate::build_project::build_project_result::BuildProjectResult;
use crate::holder::Holder;

#[derive(Clone, Default)]
pub struct BuildProjectResultHolder {
    rhai_template_renderer: Arc<RwLock<Option<BuildProjectResult>>>,
    pub update_notifier: Arc<Notify>,
}

#[async_trait]
impl Holder for BuildProjectResultHolder {
    type Item = BuildProjectResult;

    fn rw_lock(&self) -> Arc<RwLock<Option<Self::Item>>> {
        self.rhai_template_renderer.clone()
    }

    fn update_notifier(&self) -> Arc<Notify> {
        self.update_notifier.clone()
    }
}
