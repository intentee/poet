use std::sync::Arc;

use tokio::sync::Notify;
use tokio::sync::RwLock;

use crate::poet_config::PoetConfig;

#[derive(Clone, Default)]
pub struct PoetConfigHolder {
    poet_config: Arc<RwLock<Option<PoetConfig>>>,
    pub update_notifier: Arc<Notify>,
}

impl PoetConfigHolder {
    pub async fn get_poet_config(&self) -> Option<PoetConfig> {
        let poet_config_opt = self.poet_config.read().await;

        poet_config_opt.clone()
    }

    pub async fn set_poet_config(&self, poet_config: Option<PoetConfig>) {
        {
            let mut poet_config_shared_writer = self.poet_config.write().await;

            *poet_config_shared_writer = poet_config;
        }

        self.update_notifier.notify_waiters();
    }
}
