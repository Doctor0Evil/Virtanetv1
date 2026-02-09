use crate::governance_meta::{ShardGovernanceMeta, HitlLevel};
use crate::stake::StakeShard; // from your existing bindings
use crate::riskofharm::RiskOfHarm; // RoH wrapper
use crate::hitl_typestate::{HitlState, PendingReview, ApprovedByHuman}; // see §3

pub struct GovernanceGuard<'a> {
    pub stake: &'a StakeShard,
    pub roh: &'a RiskOfHarm,
}

pub enum GovernanceDecision {
    Allowed,
    RequiresHitlGate,
    Rejected(String),
}

impl<'a> GovernanceGuard<'a> {
    pub fn evaluate_route(
        &self,
        gov: &ShardGovernanceMeta,
        caller_roles: &[String],
        roh_before: f32,
        roh_after: f32,
    ) -> GovernanceDecision {
        // 1. RoH monotone + ceiling (reuses your existing pattern).
        if roh_after > self.roh.roh_ceiling() + f32::EPSILON {
            return GovernanceDecision::Rejected("ROH_LIMIT".into());
        }
        if roh_after > roh_before + 1e-6 {
            return GovernanceDecision::Rejected("ROH_MONOTONE_VIOLATION".into());
        }

        // 2. Role-based constraints (AC-3).
        if !self
            .stake
            .roles_satisfy_constraints(&gov.role_constraints, caller_roles)
        {
            return GovernanceDecision::Rejected("STAKERolesMissing".into());
        }

        // 3. HITL requirement.
        match gov.human_primacy.hitl_level {
            HitlLevel::HighImpactRequired => GovernanceDecision::RequiresHitlGate,
            _ => GovernanceDecision::Allowed,
        }
    }
}
