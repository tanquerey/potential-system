pub mod interceptor;
pub mod mission_entry;
pub mod patrol_drone;
pub mod agent_type;
pub mod target;

pub use interceptor::Interceptor;
pub use mission_entry::MissionEntry;
pub use patrol_drone::PatrolDrone;
pub use agent_type::AgentType;
use tokio::time::Instant;

use crate::{agents::patrol_drone::EngineError, event::MissionEvent};

pub trait Agent {
    fn act(&mut self, fused_input: &MissionEvent) -> Result<MissionEntry, EngineError>;
    fn id(&self) -> u8; // helper so Coordinator can tag entries
    fn last_seen(&self) -> Option<Instant> { None } // default: agents that don't track this just opt out
}
