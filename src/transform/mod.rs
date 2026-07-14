pub mod decision;
pub mod resilient;
pub mod simple;

use crate::{agents::patrol_drone::Target, event::MissionEvent};
pub trait FusionModule {
    fn fuse(&self, inputs: Vec<String>) -> String;
}

pub trait DecisionModule {
    fn decide(&self, inputs: Vec<&Target>) -> MissionEvent;
}

#[derive(Debug, Clone)]
pub enum FusedMessage {
    Radar(String),
    Camera(String),
    Combined(String),
}
