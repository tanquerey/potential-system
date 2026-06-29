#[derive(Debug, Clone)]
pub enum MissionEvent {
    Radar(String),
    Camera(String),
    Alert(String),
    Idle,
}
