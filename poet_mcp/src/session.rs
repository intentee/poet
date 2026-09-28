use crate::session_notifier::SessionNotifier;
use crate::session_subscriptions::SessionSubscriptions;

#[derive(Clone)]
pub struct Session {
    pub id: String,
    pub notifier: SessionNotifier,
    pub subscriptions: SessionSubscriptions,
}
