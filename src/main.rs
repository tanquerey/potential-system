mod agents;
mod coordinator;
mod fusion;

use std::time::Duration;

use crate::agents::{Analyzer, Interceptor, PatrolDrone};
use crate::coordinator::Coordinator;
use crate::fusion::FusedMessage;
use tokio::sync::broadcast;

#[tokio::main]
async fn main() {
    let (radar_tx, radar_rx) = broadcast::channel(16);
    let (camera_tx, camera_rx) = broadcast::channel(16);

    let mut coordinator = Coordinator::new(5);
    coordinator.add_agent(Box::new(PatrolDrone {
        id: 1,
        radar_count: 0,
    }));
    coordinator.add_agent(Box::new(Interceptor {
        id: 2,
        last_radar_count: 0,
    }));
    coordinator.add_agent(Box::new(Analyzer { id: 3 }));

    // let idle_timeout = coordinator.idle_timeout(); // getter method
    let (agents, mission_log) = coordinator.into_parts();

    Coordinator::spawn(
        radar_rx,
        camera_rx,
        agents,
        mission_log.clone(),
        Duration::new(5, 0),
    );

    tokio::spawn(async move {
        loop {
            radar_tx.send(FusedMessage::Radar("Ping".into())).unwrap();
            camera_tx
                .send(FusedMessage::Camera("Target".into()))
                .unwrap();
            tokio::time::sleep(std::time::Duration::from_secs(2)).await;
        }
    });

    tokio::time::sleep(std::time::Duration::from_secs(10)).await;

    // inspect mission log after run
    let log = mission_log.lock().unwrap();
    for entry in log.iter() {
        println!(
            "[{:?}] Agent {}: {}",
            entry.timestamp, entry.agent_id, entry.message
        );
    }
}
