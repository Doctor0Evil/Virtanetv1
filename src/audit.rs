use crate::domain::{RoutingActionKind, SecurityZone};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};
use thiserror::Error;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct AuditEvent {
    pub event_id: String,
    pub session_id: String,
    pub command_id: String,
    pub actor_did: String,
    pub zones: (SecurityZone, SecurityZone),
    pub action: RoutingActionKind,
    pub magnitude_mw: f64,
    pub approved_by_human_did: Option<String>,
    pub decision: String,
    pub timestamp: DateTime<Utc>,
    pub hash_prev: Option<String>,
    pub hash_self: String,
}

#[derive(Error, Debug)]
pub enum AuditError {
    #[error("storage error: {0}")]
    Storage(String),
}

pub trait AuditLog: Send + Sync {
    fn append(&self, event: AuditEvent) -> Result<(), AuditError>;
    fn get_by_session(&self, session_id: &str) -> Result<Vec<AuditEvent>, AuditError>;
}

pub struct InMemoryAuditLog {
    inner: Arc<Mutex<Vec<AuditEvent>>>,
}

impl InMemoryAuditLog {
    pub fn new() -> Self {
        Self {
            inner: Arc::new(Mutex::new(Vec::new())),
        }
    }

    fn compute_hash(prev: Option<&AuditEvent>, ev: &AuditEvent) -> String {
        let prev_hash = prev.map(|e| e.hash_self.clone()).unwrap_or_default();
        let payload = format!(
            "{}|{}|{}|{}|{}|{}|{}|{}",
            prev_hash,
            ev.event_id,
            ev.session_id,
            ev.command_id,
            ev.actor_did,
            ev.magnitude_mw,
            ev.decision,
            ev.timestamp
        );
        format!("{:x}", md5::compute(payload))
    }
}

impl AuditLog for InMemoryAuditLog {
    fn append(&self, mut event: AuditEvent) -> Result<(), AuditError> {
        let mut guard = self.inner.lock().map_err(|e| AuditError::Storage(e.to_string()))?;
        let prev = guard.last().cloned();
        event.hash_prev = prev.as_ref().map(|e| e.hash_self.clone());
        event.hash_self = Self::compute_hash(prev.as_ref(), &event);
        guard.push(event);
        Ok(())
    }

    fn get_by_session(&self, session_id: &str) -> Result<Vec<AuditEvent>, AuditError> {
        let guard = self.inner.lock().map_err(|e| AuditError::Storage(e.to_string()))?;
        Ok(guard
            .iter()
            .filter(|e| e.session_id == session_id)
            .cloned()
            .collect())
    }
}
