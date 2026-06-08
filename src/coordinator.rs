use crate::sensors::Sensor;
use crate::fusion::FusionModule;
use tokio::sync::broadcast;

pub struct Coordinator<F: FusionModule> {
    sensors: Vec<Box<dyn Sensor>>,
    fusion: F,
    tx: broadcast::Sender<String>,
}

impl<F: FusionModule> Coordinator<F> {
    pub fn new(sensors: Vec<Box<dyn Sensor>>, fusion: F, tx: broadcast::Sender<String>) -> Self {
        Coordinator { sensors, fusion, tx }
    }

    pub async fn run(&self) {
        // let mut tick = 0;
        loop {
            // tick += 1;
            let inputs: Vec<String> = self.sensors.iter().map(|s| s.read()).collect();
            let fused = self.fusion.fuse(inputs);
            println!("Coordinator fused: {}", fused);

            if self.tx.send(fused).is_err() {
                println!("No agents listening, stopping coordinator.");
                break;
            }

            tokio::time::sleep(std::time::Duration::from_secs(2)).await;
        }
    }
}
