use crate::sensors::Sensor;
use crate::fusion::{FusionModule, FusedMessage};
use tokio::sync::broadcast;
use tokio::time::{sleep, Duration};

pub struct Coordinator<F: FusionModule> {
    sensors: Vec<Box<dyn Sensor>>,
    fusion: F,
    tx: broadcast::Sender<FusedMessage>,
}

impl<F: FusionModule> Coordinator<F> {
    pub fn new(sensors: Vec<Box<dyn Sensor>>, fusion: F, tx: broadcast::Sender<FusedMessage>) -> Self {
        Coordinator { sensors, fusion, tx }
    }

    pub async fn run(&self) {
        // let mut tick = 0;
        loop {
            // tick += 1;

            let inputs: Vec<String> = self.sensors.iter().map(|s| s.read()).collect();
            let fused = self.fusion.fuse(inputs);

            // Example: tag based on content
            let msg = if fused.contains("Radar") && fused.contains("Camera") {
                FusedMessage::Combined(fused)
            } else if fused.contains("Radar") {
                FusedMessage::Radar(fused)
            } else {
                FusedMessage::Camera(fused)
            };

            println!("Coordinator fused: {:?}", msg);

            if self.tx.send(msg).is_err() {
                println!("No agents listening, stopping coordinator.");
                break;
            }

            sleep(Duration::from_secs(2)).await;
        }
    }
}
