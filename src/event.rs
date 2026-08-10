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
pub struct SensorId {
    pub id: u8,
}

pub const RADAR: SensorId = SensorId { id: 1 };
pub const CAMERA: SensorId = SensorId { id: 2 };
