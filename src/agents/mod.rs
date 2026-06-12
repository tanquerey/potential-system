pub mod patrol_drone;
pub mod interceptor;
pub mod analyzer;

pub use patrol_drone::PatrolDrone;
pub use interceptor::Interceptor;
pub use analyzer::Analyzer;

pub trait Agent: Send + 'static{
    fn act(&mut self, fused_input: &str) -> String;}
