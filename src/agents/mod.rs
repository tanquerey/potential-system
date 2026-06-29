pub mod interceptor;
pub mod mission_entry;
pub mod patrol_drone;

pub use interceptor::Interceptor;
pub use mission_entry::MissionEntry;
pub use patrol_drone::PatrolDrone;

use crate::event::MissionEvent;

pub trait Agent {
    fn act(&mut self, fused_input: &MissionEvent) -> MissionEntry;
    fn id(&self) -> u32; // helper so Coordinator can tag entries
}
