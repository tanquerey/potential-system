use crate::agents::target::{Detected, Intercepting, Tracking};

#[derive(Debug, Clone)]
pub enum MissionEvent {
    Radar(Detected),
    Camera(Detected),
    Alert(Tracking),
    Intercept(Intercepting),
    Command(String),
    Idle(String),
}
