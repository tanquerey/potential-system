pub mod patrol_drone;
pub mod interceptor;
pub mod analyzer;

pub use patrol_drone::spawn_patrol_drone;
pub use interceptor::spawn_interceptor;
pub use analyzer::spawn_analyzer;

pub trait Agent {
    fn act(&self, fused_input: &str) -> String;
}
