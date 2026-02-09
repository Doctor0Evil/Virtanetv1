use eco_infra_aln_router::{
    audit::InMemoryAuditLog,
    channel::LoggingAgentChannel,
    policy::{
        GovernanceRuleConfig, RulesEcoGovernancePolicy, SegmentationPolicy,
        StaticSegmentationPolicy,
    },
    router::EcoInfraRouter,
};
use std::fs;
use tracing_subscriber::EnvFilter;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .init();

    let seg_yaml = fs::read_to_string("config/segmentation_static.yaml")?;
    let segmentation = StaticSegmentationPolicy::from_yaml(&seg_yaml)?;

    let gov_yaml = fs::read_to_string("config/governance_rules.yaml")?;
    let rules: GovernanceRuleConfig = serde_yaml::from_str(&gov_yaml)?;
    let governance = RulesEcoGovernancePolicy::new(rules);

    let audit_log = InMemoryAuditLog::new();
    let channel = LoggingAgentChannel;

    let _router = EcoInfraRouter::new(segmentation, governance, audit_log, channel);

    tracing::info!("eco_infra_routerd started");

    loop {
        tokio::time::sleep(std::time::Duration::from_secs(60)).await;
    }
}
