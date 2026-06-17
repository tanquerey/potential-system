pub mod patrol_drone;
pub mod interceptor;
pub mod analyzer;
pub mod mission_entry;

pub use patrol_drone::PatrolDrone;
pub use interceptor::Interceptor;
pub use analyzer::Analyzer;
pub use mission_entry::MissionEntry;

pub trait Agent: Send + 'static {
    fn act<'a>(&mut self, fused_input: &'a str) -> MissionEntry;
    fn id(&self) -> u32; // helper so Coordinator can tag entries
}

