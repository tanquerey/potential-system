#[derive(Debug, Clone)]
pub enum MissionEvent {
    Radar(u32),
    Intercept(u32),
    Camera(u32),
    Alert(u32),
    Command(String),
    Idle(String),
}
