use std::sync::Arc;
use std::sync::RwLock;
use std::sync::RwLockReadGuard;
use std::sync::RwLockWriteGuard;

use tokio::sync::Notify;

use crate::holder_state::HolderState;

pub struct Holder<TItem> {
    state: Arc<RwLock<HolderState<TItem>>>,
    pub update_notifier: Arc<Notify>,
}

impl<TItem: Clone> Holder<TItem> {
    #[must_use]
    pub fn get(&self) -> HolderState<TItem> {
        self.read_state().clone()
    }

    pub fn reset(&self) {
        self.replace_state(HolderState::NotReady);
    }

    pub fn set(&self, item: TItem) {
        self.replace_state(HolderState::Ready(item));
    }

    #[expect(
        clippy::expect_used,
        reason = "a poisoned lock means another thread panicked while replacing the held item"
    )]
    fn read_state(&self) -> RwLockReadGuard<'_, HolderState<TItem>> {
        self.state.read().expect("Holder lock is poisoned")
    }

    fn replace_state(&self, state: HolderState<TItem>) {
        *self.write_state() = state;
        self.update_notifier.notify_waiters();
    }

    #[expect(
        clippy::expect_used,
        reason = "a poisoned lock means another thread panicked while replacing the held item"
    )]
    fn write_state(&self) -> RwLockWriteGuard<'_, HolderState<TItem>> {
        self.state.write().expect("Holder lock is poisoned")
    }
}

impl<TItem> Clone for Holder<TItem> {
    fn clone(&self) -> Self {
        Self {
            state: self.state.clone(),
            update_notifier: self.update_notifier.clone(),
        }
    }
}

impl<TItem> Default for Holder<TItem> {
    fn default() -> Self {
        Self {
            state: Arc::new(RwLock::new(HolderState::NotReady)),
            update_notifier: Arc::default(),
        }
    }
}
