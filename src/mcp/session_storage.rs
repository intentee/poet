use actix_web::Result;
use dashmap::DashMap;

use crate::mcp::session::Session;

#[derive(Clone, Default)]
pub struct SessionStorage {
    sessions: DashMap<String, Session>,
}

impl SessionStorage {
    pub async fn read(&self, session_id: &str) -> Result<Option<Session>> {
        Ok(self.sessions.get(session_id).map(|session| session.clone()))
    }

    pub async fn store_new_session(&self, session: Session) -> Result<()> {
        self.update_session(session).await
    }

    pub async fn terminate_session(&self, session: Session) -> Result<()> {
        self.sessions.remove(&session.session_id);

        Ok(())
    }

    pub async fn update_session(&self, session: Session) -> Result<()> {
        self.sessions.insert(session.session_id.clone(), session);

        Ok(())
    }
}
