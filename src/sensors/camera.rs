use super::Sensor;

pub struct CameraSensor;

impl Sensor for CameraSensor {
    fn read(&self) -> String { "Camera frame".to_string() }
}
