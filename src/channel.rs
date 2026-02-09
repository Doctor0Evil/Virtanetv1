use crate::domain::RoutingCommand;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ChannelError {
    #[error("transport: {0}")]
    Transport(String),
}

pub trait AgentChannel: Send + Sync {
    fn send_routing_command(&self, cmd: &RoutingCommand) -> Result<(), ChannelError>;
}

pub struct LoggingAgentChannel;

impl AgentChannel for LoggingAgentChannel {
    fn send_routing_command(&self, cmd: &RoutingCommand) -> Result<(), ChannelError> {
        tracing::info!(
            command_id = %cmd.id,
            session_id = %cmd.session_id,
            source = %cmd.source.0,
            target = %cmd.target.0,
            action = ?cmd.action,
            magnitude_mw = cmd.magnitude_mw,
            "Dispatching routing command over LoggingAgentChannel"
        );
        Ok(())
    }
}
