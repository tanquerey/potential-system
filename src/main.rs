mod agents;
mod coordinator;
mod event;
mod transform;
use std::time::Instant;

use crate::agents::patrol_drone::Target;
use crate::agents::{PatrolDrone};
use crate::coordinator::Coordinator;
use crate::event::MissionEvent;
use tokio::sync::mpsc;

#[tokio::main]
async fn main() {
    // Channel for mission events
    let (tx, rx) = mpsc::channel(32);

    // Create coordinator with sender
    let mut coordinator = Coordinator::new();

    // Add a patrol drone agent
    let dummy_target1 = Target {
        target_dist: 0,            // placeholder distance
        last_seen: Instant::now(), // current time as dummy
    };
    let dummy_target2 = Target {
        target_dist: 0,            // placeholder distance
        last_seen: Instant::now(), // current time as dummy
    };
    let drone = PatrolDrone {
        id: 1,
        radar_count: 0,
        radar_count_last_reset: Instant::now(),
        target_detected_from_camera: dummy_target1,
        target_detected_from_radar: dummy_target2,
    };

    coordinator.add_agent(Box::new(drone));

    // Spawn coordinator task
    let coordinator_task = tokio::spawn(async move {
        coordinator.run(rx).await;
    });

    // Dispatch some events
    tx.send(MissionEvent::Radar(20)).await.unwrap();
    tx.send(MissionEvent::Camera(20)).await.unwrap();
    // tx.send(MissionEvent::Radar(16)).await.unwrap();
    // tx.send(MissionEvent::Radar(17)).await.unwrap();
    tx.send(MissionEvent::Idle("test idle".into()))
        .await
        .unwrap();
    coordinator_task.await.unwrap();
}
