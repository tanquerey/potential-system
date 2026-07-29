pub mod decision;
pub mod confidence;

use flux_perception::Engine;

use crate::{event::MissionEvent};

pub trait DecisionModule {
    fn decide(&self, engine : &Engine) -> MissionEvent;
}