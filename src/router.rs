use crate::audit::{AuditEvent, AuditLog};
use crate::channel::AgentChannel;
use crate::domain::{RoutingCommand};
use crate::policy::{EcoGovernancePolicy, SegmentationPolicy};
use chrono::Utc;
use thiserror::Error;
use uuid::Uuid;

#[derive(Error, Debug)]
pub enum RouterError {
    #[error("segmentation: {0}")]
    Segmentation(String),
    #[error("governance: {0}")]
    Governance(String),
    #[error("audit: {0}")]
    Audit(String),
    #[error("channel: {0}")]
    Channel(String),
}

pub struct EcoInfraRouter<S, G, L, C>
where
    S: SegmentationPolicy,
    G: EcoGovernancePolicy,
    L: AuditLog,
    C: AgentChannel,
{
    segmentation: S,
    governance: G,
    audit_log: L,
    channel: C,
}

impl<S, G, L, C> EcoInfraRouter<S, G, L, C>
where
    S: SegmentationPolicy,
    G: EcoGovernancePolicy,
    L: AuditLog,
    C: AgentChannel,
{
    pub fn new(segmentation: S, governance: G, audit_log: L, channel: C) -> Self {
        Self {
            segmentation,
            governance,
            audit_log,
            channel,
        }
    }

    pub fn route(&self, cmd: RoutingCommand) -> Result<(), RouterError> {
        let src_zone = self
            .segmentation
            .node_zone(&cmd.source)
            .ok_or_else(|| RouterError::Segmentation("Unknown source zone".into()))?;
        let dst_zone = self
            .segmentation
            .node_zone(&cmd.target)
            .ok_or_else(|| RouterError::Segmentation("Unknown target zone".into()))?;

        if !self.segmentation.is_command_permitted(&cmd) {
            let event = AuditEvent {
                event_id: Uuid::new_v4().to_string(),
                session_id: cmd.session_id.clone(),
                command_id: cmd.id.clone(),
                actor_did: cmd.issued_by.did.clone(),
                zones: (src_zone, dst_zone),
                action: cmd.action.clone(),
                magnitude_mw: cmd.magnitude_mw,
                approved_by_human_did: None,
                decision: "DENIED_SEGMENTATION".into(),
                timestamp: Utc::now(),
                hash_prev: None,
                hash_self: String::new(),
            };
            self.audit_log
                .append(event)
                .map_err(|e| RouterError::Audit(e.to_string()))?;
            return Err(RouterError::Segmentation(
                "Command denied by segmentation policy".into(),
            ));
        }

        self.governance
            .validate_routing(&cmd)
            .map_err(|e| RouterError::Governance(e))?;

        let requires_hitl = self.governance.requires_human_approval(&cmd);
        let decision = if requires_hitl {
            "PENDING_HITL"
        } else {
            "ALLOWED"
        }
        .to_string();

        let event = AuditEvent {
            event_id: Uuid::new_v4().to_string(),
            session_id: cmd.session_id.clone(),
            command_id: cmd.id.clone(),
            actor_did: cmd.issued_by.did.clone(),
            zones: (src_zone, dst_zone),
            action: cmd.action.clone(),
            magnitude_mw: cmd.magnitude_mw,
            approved_by_human_did: None,
            decision: decision.clone(),
            timestamp: Utc::now(),
            hash_prev: None,
            hash_self: String::new(),
        };
        self.audit_log
            .append(event)
            .map_err(|e| RouterError::Audit(e.to_string()))?;

        if requires_hitl {
            return Ok(());
        }

        self.channel
            .send_routing_command(&cmd)
            .map_err(|e| RouterError::Channel(e.to_string()))
    }
}
