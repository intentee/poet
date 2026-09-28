use std::sync::Arc;

use dashmap::DashMap;
use dashmap::Entry;
use tokio_util::sync::CancellationToken;

use crate::mcp_error::McpError;

#[derive(Clone, Default)]
pub struct SessionSubscriptions {
    cancellation_tokens: Arc<DashMap<String, CancellationToken>>,
}

impl SessionSubscriptions {
    pub fn cancel_all(&self) {
        for cancellation_token in self.cancellation_tokens.iter() {
            cancellation_token.value().cancel();
        }
    }

    pub fn subscribe(&self, uri: &str) -> Result<CancellationToken, McpError> {
        match self.cancellation_tokens.entry(uri.to_owned()) {
            Entry::Occupied(occupied_entry) if !occupied_entry.get().is_cancelled() => {
                Err(McpError::AlreadySubscribed {
                    uri: uri.to_owned(),
                })
            }
            Entry::Occupied(mut occupied_entry) => {
                let cancellation_token = CancellationToken::new();

                occupied_entry.insert(cancellation_token.clone());

                Ok(cancellation_token)
            }
            Entry::Vacant(vacant_entry) => {
                Ok(vacant_entry.insert(CancellationToken::new()).clone())
            }
        }
    }

    pub fn unsubscribe(&self, uri: &str) {
        if let Some((_, cancellation_token)) = self.cancellation_tokens.remove(uri) {
            cancellation_token.cancel();
        }
    }
}
