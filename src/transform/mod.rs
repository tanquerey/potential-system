pub mod decision;
pub mod resilient;
pub mod simple;
pub mod confidence;

use flux_perception::Engine;

use crate::{agents::patrol_drone::Target, event::MissionEvent};
pub trait FusionModule {
    fn fuse(&self, inputs: Vec<String>) -> String;
}

pub trait DecisionModule {
    fn decide(&self, engine : &Engine) -> MissionEvent;
}

#[derive(Debug, Clone)]
pub enum FusedMessage {
    Radar(String),
    Camera(String),
    Combined(String),
}
