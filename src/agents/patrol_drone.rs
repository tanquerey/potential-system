use tokio::sync::broadcast;

use crate::{agents::Agent, fusion::FusedMessage};

pub struct PatrolDrone {
    pub id: u32,
    pub radar_count: u32,
}

impl Agent for PatrolDrone {
    fn act(&mut self, fused_input: &str) -> String {
        if fused_input.contains("Radar") {
            self.radar_count += 1;
            if self.radar_count >= 3 {
                self.radar_count = 0;
                format!("PatrolDrone {} ALERT: multiple radar detections!", self.id)
            } else {
                format!("PatrolDrone {} responding to {}", self.id, fused_input)
            }
        } else {
            format!("PatrolDrone {} idle", self.id)
        }
    }
}

fn spawn(mut rx: tokio::sync::broadcast::Receiver<FusedMessage>, id: u32) {
    tokio::spawn(async move {
        let mut drone = PatrolDrone { id, radar_count: 0 };
        while let Ok(msg) = rx.recv().await {
            let fused_str = format!("{:?}", msg);
            println!("{}", drone.act(&fused_str));
        }
    });
}
