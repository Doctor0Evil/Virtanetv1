use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct AiRmfTag {
    pub functions: Vec<String>,        // ["Map","Manage"]
    pub hazard_class: Option<String>,  // "ot-actuation-misroute"
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Sp80053Ref {
    pub family: String,        // "AC"
    pub controls: Vec<String>, // ["AC-2","AC-3"]
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Sp80053Impact {
    pub confidentiality: String, // "Low|Moderate|High"
    pub integrity: String,
    pub availability: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Web5PqcProfile {
    pub did: String,
    pub dwn_locations: Vec<String>,
    pub kem_profile: String, // "ML-KEM-768"
    pub sig_profile: String, // "ML-DSA-87"
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum HitlLevel {
    None,
    LowImpactOptional,
    HighImpactRequired,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct HumanPrimacy {
    pub hitl_level: HitlLevel,
    pub appeal_path_id: Option<String>, // ref into appeal-routes shard
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct RoleConstraint {
    pub role_name: String,       // "EcoCouncil","OTOperator"
    pub min_signatures: u8,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ShardGovernanceMeta {
    pub ai_rmf: AiRmfTag,
    pub sp800_53_refs: Vec<Sp80053Ref>,
    pub sp800_53_impact: Sp80053Impact,
    pub web5_pqc: Web5PqcProfile,
    pub human_primacy: HumanPrimacy,
    pub role_constraints: Vec<RoleConstraint>,
}

impl ShardGovernanceMeta {
    pub fn requires_hitl(&self) -> bool {
        matches!(self.human_primacy.hitl_level, HitlLevel::HighImpactRequired)
    }
}
