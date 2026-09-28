use std::sync::Arc;

use tokio::sync::watch::Receiver;
use tokio::sync::watch::Sender;
use tokio::sync::watch::channel;

use crate::holder_state::HolderState;

pub struct Holder<TItem> {
    state_sender: Arc<Sender<HolderState<TItem>>>,
}

impl<TItem: Clone> Holder<TItem> {
    #[must_use]
    pub fn get(&self) -> HolderState<TItem> {
        self.state_sender.borrow().clone()
    }

    pub fn reset(&self) {
        self.state_sender.send_replace(HolderState::NotReady);
    }

    pub fn set(&self, item: TItem) {
        self.state_sender.send_replace(HolderState::Ready(item));
    }

    #[must_use]
    pub fn subscribe(&self) -> Receiver<HolderState<TItem>> {
        self.state_sender.subscribe()
    }
}

impl<TItem> Clone for Holder<TItem> {
    fn clone(&self) -> Self {
        Self {
            state_sender: self.state_sender.clone(),
        }
    }
}

impl<TItem> Default for Holder<TItem> {
    fn default() -> Self {
        let (state_sender, _state_receiver) = channel(HolderState::NotReady);

        Self {
            state_sender: Arc::new(state_sender),
        }
    }
}
