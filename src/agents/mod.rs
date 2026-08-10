pub mod interceptor;
pub mod mission_entry;
pub mod patrol_drone;
pub mod agent_type;
pub mod target;

pub use interceptor::Interceptor;
pub use mission_entry::MissionEntry;
pub use patrol_drone::PatrolDrone;
pub use agent_type::AgentType;

use crate::{agents::patrol_drone::EngineError, event::MissionEvent};

pub trait Agent {
    fn act(&mut self, fused_input: &MissionEvent) -> Result<MissionEntry, EngineError>;
    fn id(&self) -> u32; // helper so Coordinator can tag entries
}
