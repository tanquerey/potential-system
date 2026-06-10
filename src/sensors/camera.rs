use super::Sensor;
use rand::Rng;

pub struct CameraSensor;

impl Sensor for CameraSensor {
    fn read(&self) -> String {
        let mut rng = rand::thread_rng();
        if rng.gen_bool(0.3) { // 30% chance
            "Camera noise".to_string()
        } else {
            "Camera frame".to_string()
        }
    }
}
