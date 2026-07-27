mod agents;
mod coordinator;
mod event;
mod transform;
use std::time::Instant;

use crate::agents::PatrolDrone;
use crate::coordinator::Coordinator;
use crate::event::MissionEvent;
use flux_perception::Engine;
use tokio::sync::mpsc;

#[tokio::main]
async fn main() {
    // Channel for mission events
    let (tx, rx) = mpsc::channel(3);

    // Create coordinator with sender
    let mut coordinator = Coordinator::new();

    let drone = PatrolDrone {
        id: 1,
        radar_count: 0,
        radar_count_last_reset: Instant::now(),
    };

    coordinator.add_agent(Box::new(drone));

    let mut engine = Engine::new(0.8);
    engine.add_sensor(1, 0.3, 0.0); // radar: higher weight
    engine.add_sensor(2, 0.7, 0.0); // camera

    // Spawn coordinator task
    let coordinator_task = tokio::spawn(async move {
        coordinator.run(rx, &mut engine).await;
    });

    // Dispatch some events
    tx.send(MissionEvent::Idle("test idle1".into()))
        .await
        .unwrap();
    tx.send(MissionEvent::Radar(15.0)).await.unwrap();
    tx.send(MissionEvent::Camera(25.0)).await.unwrap();
    // tx.send(MissionEvent::Radar(16)).await.unwrap();
    // tx.send(MissionEvent::Radar(17)).await.unwrap();
    tx.send(MissionEvent::Idle("test idle2".into()))
        .await
        .unwrap();
    coordinator_task.await.unwrap();
}
