use crate::domain::*;
use serde::Deserialize;
use std::collections::HashMap;

pub trait SegmentationPolicy: Send + Sync {
    fn node_zone(&self, node: &AlnNodeId) -> Option<SecurityZone>;
    fn device_class(&self, node: &AlnNodeId) -> Option<DeviceClass>;
    fn is_command_permitted(&self, cmd: &RoutingCommand) -> bool;
}

#[derive(Debug, Deserialize)]
struct NodeConfig {
    id: String,
    zone: SecurityZone,
    device_class: DeviceClass,
}

#[derive(Debug, Deserialize)]
pub struct StaticSegmentationConfig {
    nodes: Vec<NodeConfig>,
}

pub struct StaticSegmentationPolicy {
    zones: HashMap<AlnNodeId, SecurityZone>,
    classes: HashMap<AlnNodeId, DeviceClass>,
}

impl StaticSegmentationPolicy {
    pub fn from_yaml(yaml: &str) -> anyhow::Result<Self> {
        let cfg: StaticSegmentationConfig = serde_yaml::from_str(yaml)?;
        let mut zones = HashMap::new();
        let mut classes = HashMap::new();

        for n in cfg.nodes {
            let id = AlnNodeId(n.id);
            zones.insert(id.clone(), n.zone);
            classes.insert(id, n.device_class);
        }

        Ok(Self { zones, classes })
    }
}

impl SegmentationPolicy for StaticSegmentationPolicy {
    fn node_zone(&self, node: &AlnNodeId) -> Option<SecurityZone> {
        self.zones.get(node).cloned()
    }

    fn device_class(&self, node: &AlnNodeId) -> Option<DeviceClass> {
        self.classes.get(node).cloned()
    }

    fn is_command_permitted(&self, cmd: &RoutingCommand) -> bool {
        let src_zone = match self.node_zone(&cmd.source) {
            Some(z) => z,
            None => return false,
        };
        let dst_zone = match self.node_zone(&cmd.target) {
            Some(z) => z,
            None => return false,
        };

        if let Some(dc) = self.device_class(&cmd.target) {
            match dc {
                DeviceClass::ApprovedCpuOnly
                | DeviceClass::SecureHsmController
                | DeviceClass::QuantumSafeGateway => {}
            }
        } else {
            return false;
        }

        match cmd.action {
            RoutingActionKind::ReconfigurePath => matches!(src_zone, SecurityZone::EcoCore),
            _ => true,
        }
    }
}

pub trait EcoGovernancePolicy: Send + Sync {
    fn validate_routing(&self, cmd: &RoutingCommand) -> Result<(), String>;
    fn requires_human_approval(&self, cmd: &RoutingCommand) -> bool;
}

#[derive(Debug, Deserialize)]
pub struct GovernanceRuleConfig {
    pub max_shed_load_mw: f64,
    pub max_step_change_mw: f64,
    pub hitl_threshold_mw: f64,
    pub protected_zones: Vec<SecurityZone>,
}

pub struct RulesEcoGovernancePolicy {
    rules: GovernanceRuleConfig,
}

impl RulesEcoGovernancePolicy {
    pub fn new(rules: GovernanceRuleConfig) -> Self {
        Self { rules }
    }
}

impl EcoGovernancePolicy for RulesEcoGovernancePolicy {
    fn validate_routing(&self, cmd: &RoutingCommand) -> Result<(), String> {
        if cmd.magnitude_mw.is_sign_negative() {
            return Err("Negative magnitude not allowed".into());
        }

        match cmd.action {
            RoutingActionKind::ShedLoadMw => {
                if cmd.magnitude_mw > self.rules.max_shed_load_mw {
                    return Err("Load shed exceeds policy limit".into());
                }
            }
            _ => {}
        }

        if cmd.magnitude_mw > self.rules.max_step_change_mw {
            return Err("Step change exceeds policy limit".into());
        }

        Ok(())
    }

    fn requires_human_approval(&self, cmd: &RoutingCommand) -> bool {
        cmd.magnitude_mw >= self.rules.hitl_threshold_mw
    }
}
