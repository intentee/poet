use dashmap::DashMap;

use crate::session::Session;

#[derive(Default)]
pub struct SessionStorage {
    pub sessions: DashMap<String, Session>,
}

impl SessionStorage {
    #[must_use]
    pub fn read(&self, session_id: &str) -> Option<Session> {
        self.sessions
            .get(session_id)
            .map(|stored_session| stored_session.value().clone())
    }

    pub fn store(&self, session: Session) {
        self.sessions.insert(session.id.clone(), session);
    }

    pub fn terminate(&self, session_id: &str) {
        if let Some((_, terminated_session)) = self.sessions.remove(session_id) {
            terminated_session.subscriptions.cancel_all();
        }
    }
}
