use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct AlnNodeId(pub String);

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct DidIdentity {
    pub did: String,
    pub verifiable_cred_id: String,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum SecurityZone {
    EcoCore,
    GridOT,
    BuildingOT,
    DataCenter,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum AssetType {
    Substation,
    Microgrid,
    SolarFarm,
    WindFarm,
    BatteryCluster,
    DataCenter,
    BuildingCluster,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum DeviceClass {
    ApprovedCpuOnly,
    SecureHsmController,
    QuantumSafeGateway,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum RoutingActionKind {
    IncreaseExportMw,
    DecreaseExportMw,
    ShedLoadMw,
    ChargeStorageMw,
    DischargeStorageMw,
    ReconfigurePath,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct RoutingCommand {
    pub id: String,
    pub session_id: String,
    pub issued_by: DidIdentity,
    pub source: AlnNodeId,
    pub target: AlnNodeId,
    pub action: RoutingActionKind,
    pub magnitude_mw: f64,
    pub reason_code: String,
    pub created_at: DateTime<Utc>,
}
