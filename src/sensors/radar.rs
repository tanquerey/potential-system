use super::Sensor;

pub struct RadarSensor;

impl Sensor for RadarSensor {
    fn read(&self) -> String { "Radar ping".to_string() }
}
