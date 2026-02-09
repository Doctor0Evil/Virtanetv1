use crate::domain::DidIdentity;
use chrono::{DateTime, Duration, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use thiserror::Error;
use uuid::Uuid;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SecureSession {
    pub id: String,
    pub actor: DidIdentity,
    pub created_at: DateTime<Utc>,
    pub expires_at: DateTime<Utc>,
}

#[derive(Error, Debug)]
pub enum SessionError {
    #[error("not found")]
    NotFound,
    #[error("expired")]
    Expired,
    #[error("storage: {0}")]
    Storage(String),
}

pub trait SessionManager: Send + Sync {
    fn issue_session(&self, actor: DidIdentity) -> Result<SecureSession, SessionError>;
    fn validate_session(&self, session_id: &str) -> Result<SecureSession, SessionError>;
}

pub struct InMemorySessionManager {
    inner: Arc<Mutex<HashMap<String, SecureSession>>>,
    ttl: Duration,
}

impl InMemorySessionManager {
    pub fn new(ttl_minutes: i64) -> Self {
        Self {
            inner: Arc::new(Mutex::new(HashMap::new())),
            ttl: Duration::minutes(ttl_minutes),
        }
    }
}

impl SessionManager for InMemorySessionManager {
    fn issue_session(&self, actor: DidIdentity) -> Result<SecureSession, SessionError> {
        let now = Utc::now();
        let sess = SecureSession {
            id: Uuid::new_v4().to_string(),
            actor,
            created_at: now,
            expires_at: now + self.ttl,
        };
        let mut guard = self.inner.lock().map_err(|e| SessionError::Storage(e.to_string()))?;
        guard.insert(sess.id.clone(), sess.clone());
        Ok(sess)
    }

    fn validate_session(&self, session_id: &str) -> Result<SecureSession, SessionError> {
        let guard = self.inner.lock().map_err(|e| SessionError::Storage(e.to_string()))?;
        let sess = guard.get(session_id).cloned().ok_or(SessionError::NotFound)?;
        if sess.expires_at < Utc::now() {
            return Err(SessionError::Expired);
        }
        Ok(sess)
    }
}
