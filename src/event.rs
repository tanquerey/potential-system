#[derive(Debug, Clone)]
pub enum MissionEvent {
    Radar(f64),
    Intercept(f64),
    Camera(f64),
    Alert(f64),
    Command(String),
    Idle(String),
}
