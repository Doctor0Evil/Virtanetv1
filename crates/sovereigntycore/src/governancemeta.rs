use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct AiRmfTag {
    pub functions: Vec<String>,          // ["MAP","MEASURE","MANAGE","GOVERN"]
    pub hazard_class: Option<String>,    // e.g. "ot-actuation-misroute", "biofield-stress"
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Sp80053Ref {
    pub family: String,                  // "AC","AU","SC","RA",...
    pub controls: Vec<String>,           // ["AC-2","AC-3"]
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Sp80053Impact {
    pub confidentiality: String,         // "Low","Moderate","High"
    pub integrity: String,
    pub availability: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Web5PqcProfile {
    pub did: String,
    pub dwn_locations: Vec<String>,
    pub kem_profile: String,             // e.g. "ML-KEM-768"
    pub sig_profile: String,             // e.g. "ML-DSA-87"
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub enum HitlLevel {
    None,
    LowImpactOptional,
    HighImpactRequired,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct HumanPrimacy {
    pub hitl_level: HitlLevel,
    pub appeal_path_id: Option<String>,
}

/// New: classification of life / cognitive class this shard protects.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SpeciesNeuroclass {
    /// High-level class, e.g. "human", "non-human-animal", "synthetic-organic-hybrid".
    pub class_label: String,
    /// Optional, finer-grained descriptor, e.g. "baseline", "cybernetics-by-choice".
    pub subclass_label: Option<String>,
    /// Whether neurorights overlays MUST always be applied.
    pub neurorights_required: bool,
}

/// New: biofield / biospatial load ceiling for this shard.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct BiofieldLoadCeiling {
    /// Normalized 0.0–1.0 ceiling for combined biospatial load (heat, pollution, etc.).
    pub max_normalized_load: f32,
    /// Optional narrative / regime identifier (e.g. "smart-city-sanctuary-v1").
    pub regime: Option<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct RoleConstraint {
    pub role_name: String,   // e.g. "EcoInfraCouncil", "OTOperator", "CommunityRep"
    pub min_signatures: u8,  // equal power thresholds across roles
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ShardGovernanceMeta {
    pub airmf: AiRmfTag,
    pub sp80053_refs: Vec<Sp80053Ref>,
    pub sp80053_impact: Sp80053Impact,
    pub web5_pqc: Web5PqcProfile,
    pub human_primacy: HumanPrimacy,
    pub role_constraints: Vec<RoleConstraint>,

    /// New: species / neuro‑class this shard is protecting.
    pub species_neuroclass: SpeciesNeuroclass,

    /// New: maximum allowed biofield / biospatial load for actions under this shard.
    pub biofield_load_ceiling: BiofieldLoadCeiling,
}

impl ShardGovernanceMeta {
    pub fn requires_hitl(&self) -> bool {
        matches!(self.human_primacy.hitl_level, HitlLevel::HighImpactRequired)
    }

    /// Convenience: returns true if this shard is tagged as a sanctuary‑grade zone.
    pub fn is_sanctuary_like(&self) -> bool {
        self.species_neuroclass.neurorights_required
            && self.biofield_load_ceiling.max_normalized_load <= 0.5
    }
}
