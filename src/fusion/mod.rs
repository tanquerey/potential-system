pub mod simple;
pub mod resilient;

pub use simple::SimpleFusion;
pub use resilient::ResilientFusion;
pub trait FusionModule {
    fn fuse(&self, inputs: Vec<String>) -> String;
}

#[derive(Debug, Clone)]
pub enum FusedMessage {
    Radar(String),
    Camera(String),
    Combined(String),
}
