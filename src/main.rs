mod agents;
mod coordinator;
mod event;
mod transform;

use crate::agents::AgentType::Patrol;
use crate::agents::PatrolDrone;
use crate::coordinator::Coordinator;
use crate::event::MissionEvent;
use flux_confidence::Confidence;
use flux_perception::Engine;
use tokio::sync::mpsc;

#[tokio::main]
async fn main() {
    struct SensorId {
        id: u8,
    }
    const RADAR: SensorId = SensorId { id: 1 };
    const CAMERA: SensorId = SensorId { id: 2 };

    // Channel for mission events
    let (tx, rx) = mpsc::channel(3);

    // Create coordinator with sender
    let mut coordinator = Coordinator::new();

    let drone = PatrolDrone {
        id: 1,
        camera_confidence: Confidence::new(0.7),
        radar_confidence: Confidence::new(0.7),
    };

    coordinator.add_agent(Patrol(drone));

    let mut engine = Engine::new(0.4);
    engine.add_sensor(RADAR.id, 0.3, 0.0); // radar: higher weight
    engine.add_sensor(CAMERA.id, 0.7, 0.0); // camera

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
    tx.send(MissionEvent::Radar(30.0)).await.unwrap();
    tx.send(MissionEvent::Camera(50.0)).await.unwrap();
    tx.send(MissionEvent::Idle("test idle2".into()))
        .await
        .unwrap();
    coordinator_task.await.unwrap();
}
