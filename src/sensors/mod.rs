pub mod radar;
pub mod camera;

pub use radar::RadarSensor;
pub use camera::CameraSensor;

pub trait Sensor {
    fn read(&self) -> String;
}
