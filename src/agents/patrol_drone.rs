use tokio::sync::broadcast;

use crate::fusion::FusedMessage;

pub struct PatrolDrone {
    pub id: u32,
    pub radar_count: u32,
}

impl PatrolDrone {
    pub fn spawn(mut rx: broadcast::Receiver<FusedMessage>, id: u32) {
        tokio::spawn(async move {
            let mut drone = PatrolDrone { id, radar_count: 0 };

            while let Ok(msg) = rx.recv().await {
                if let FusedMessage::Radar(data) = msg {
                    drone.radar_count += 1;
                    println!("PatrolDrone {} saw radar: {}", drone.id, data);

                    if drone.radar_count >= 3 {
                        println!("PatrolDrone {} ALERT: multiple radar detections!", drone.id);
                        drone.radar_count = 0; // reset after alert
                    }
                }
            }
        });
    }
}
