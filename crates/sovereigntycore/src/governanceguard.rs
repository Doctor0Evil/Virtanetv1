use crate::governancemeta::{BiofieldLoadCeiling, HitlLevel, ShardGovernanceMeta};
use crate::riskofharm::RiskOfHarm;
use crate::stake::StakeShard;

#[derive(Clone, Debug)]
pub enum GovernanceDecision<T> {
    Allowed(T),
    RequiresHitlGate(T),
    Rejected(String),
}

/// Context passed in from the caller when evaluating a route / update.
#[derive(Clone, Debug)]
pub struct GovernanceContext {
    /// Role identifiers attached to the caller (from stake / VC).
    pub caller_roles: Vec<String>,
    /// Current RoH before applying the proposed change (0.0–1.0).
    pub roh_before: f32,
    /// Predicted RoH after applying the proposed change.
    pub roh_after: f32,
    /// Current normalized biofield / biospatial load (0.0–1.0).
    pub biofield_load: f32,
}

pub struct GovernanceGuard<'a> {
    pub stake: &'a StakeShard,
    pub roh: &'a RiskOfHarm,
}

impl<'a> GovernanceGuard<'a> {
    pub fn evaluate<T>(
        &self,
        meta: &ShardGovernanceMeta,
        ctx: &GovernanceContext,
        payload: T,
    ) -> GovernanceDecision<T> {
        // 1. RoH ceiling + monotone safety (no increase).
        if ctx.roh_after > self.roh.roh_ceiling() + f32::EPSILON {
            return GovernanceDecision::Rejected("ROH ceiling exceeded".into());
        }
        if ctx.roh_after > ctx.roh_before + 1e-6 {
            return GovernanceDecision::Rejected("ROH monotone safety violated".into());
        }

        // 2. Species / neurorights: sanctuary‑like zones MUST never exceed their biofield ceiling.
        if !self.enforce_biofield_ceiling(&meta.biofield_load_ceiling, ctx.biofield_load) {
            return GovernanceDecision::Rejected("biofield load ceiling exceeded".into());
        }

        // 3. Role constraints (SP 800‑53 AC‑2/AC‑3‑like behavior).
        if !self
            .stake
            .roles_satisfy_constraints(&meta.role_constraints, &ctx.caller_roles)
        {
            return GovernanceDecision::Rejected("stake role constraints not satisfied".into());
        }

        // 4. HITL requirement: for high‑impact shards or sanctuary‑like contexts,
        //    force HITL gate if configured.
        if meta.requires_hitl() && self.is_high_impact(&meta.sp80053_impact) {
            return GovernanceDecision::RequiresHitlGate(payload);
        }

        GovernanceDecision::Allowed(payload)
    }

    fn enforce_biofield_ceiling(
        &self,
        ceiling: &BiofieldLoadCeiling,
        current_load: f32,
    ) -> bool {
        current_load <= ceiling.max_normalized_load + f32::EPSILON
    }

    fn is_high_impact(&self, impact: &crate::governancemeta::Sp80053Impact) -> bool {
        impact.integrity == "High" || impact.availability == "High"
    }
}
