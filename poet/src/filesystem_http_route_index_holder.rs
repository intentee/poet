use std::sync::Arc;

use async_trait::async_trait;
use tokio::sync::Notify;
use tokio::sync::RwLock;

use crate::filesystem_http_route_index::FilesystemHttpRouteIndex;
use crate::holder::Holder;

#[derive(Clone, Default)]
pub struct FilesystemHttpRouteIndexHolder {
    filesystem_http_route_index: Arc<RwLock<Option<Arc<FilesystemHttpRouteIndex>>>>,
    pub update_notifier: Arc<Notify>,
}

#[async_trait]
impl Holder for FilesystemHttpRouteIndexHolder {
    type Item = Arc<FilesystemHttpRouteIndex>;

    fn rw_lock(&self) -> Arc<RwLock<Option<Self::Item>>> {
        self.filesystem_http_route_index.clone()
    }

    fn update_notifier(&self) -> Arc<Notify> {
        self.update_notifier.clone()
    }
}
