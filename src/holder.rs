use std::mem::replace;
use std::sync::Arc;

use async_trait::async_trait;
use tokio::sync::Notify;
use tokio::sync::RwLock;

#[async_trait]
pub trait Holder {
    type Item: Clone + Send + Sync;

    fn rw_lock(&self) -> Arc<RwLock<Option<Self::Item>>>;

    fn update_notifier(&self) -> Arc<Notify>;

    async fn get(&self) -> Option<Self::Item> {
        let rw_lock = self.rw_lock();
        let item_opt = rw_lock.read().await;

        item_opt.clone()
    }

    fn on_update(&self, _item: &Option<Self::Item>) {}

    async fn set(&self, item: Option<Self::Item>) {
        {
            let rw_lock = self.rw_lock();
            let mut item_shared_writer = rw_lock.write().await;

            self.on_update(&item);

            *item_shared_writer = item;
        }

        self.update_notifier().notify_waiters();
    }

    async fn swap(&self, item: Option<Self::Item>) -> Option<Self::Item> {
        let old_item: Option<Self::Item> = {
            let rw_lock = self.rw_lock();
            let mut item_shared_writer = rw_lock.write().await;

            self.on_update(&item);

            replace(&mut *item_shared_writer, item)
        };

        self.update_notifier().notify_waiters();

        old_item
    }
}
