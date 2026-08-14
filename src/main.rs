mod agents;
mod coordinator;
mod event;

use crate::agents::AgentType::{self};
use crate::agents::PatrolDrone;
use crate::agents::target::Detected;
use crate::coordinator::Coordinator;
use crate::event::{CAMERA, MissionEvent, RADAR, SourcedEvent};
use flux_confidence::Confidence;
use flux_perception::Engine;
use glam::Vec3;
use tokio::sync::mpsc;
use tokio::time::Instant;

#[tokio::main]
async fn main() {
    let (tx, rx) = mpsc::channel(3);
    let mut coordinator = Coordinator::new();

    let mut engine = Engine::new(0.6);
    engine.add_sensor(RADAR.id, 0.5, 0.0);
    engine.add_sensor(CAMERA.id, 0.5, 0.0);
    coordinator.add_agent(AgentType::Patrol(PatrolDrone {
        id: 1,
        engine: engine,
        current_pos: Vec3::new(0.0, 0.0, 2.0),
        camera_confidence: Confidence::new(0.7),
        radar_confidence: Confidence::new(0.7),
        last_seen: Instant::now(),
    }));

    let mut engine = Engine::new(0.5);
    engine.add_sensor(RADAR.id, 0.5, 0.0); // radar: higher weight
    engine.add_sensor(CAMERA.id, 0.5, 0.0); // camera
    coordinator.add_agent(AgentType::Patrol(PatrolDrone {
        id: 2,
        engine: engine,
        current_pos: Vec3::new(0.0, 5.0, 2.0),
        camera_confidence: Confidence::new(0.7),
        radar_confidence: Confidence::new(0.7),
        last_seen: Instant::now(),
    }));

    let coordinator_task = tokio::spawn(async move {
        coordinator.run(rx).await;
    });

    // one task per drone, each with its own cloned Sender
    let mut drone_tasks = Vec::new();
    for drone_id in 1..=2 {
        let tx_clone = tx.clone(); // new handle to the SAME channel
        let task = tokio::spawn(async move {
            tx_clone
                .send(SourcedEvent {
                    drone_id: drone_id,
                    event: MissionEvent::Radar(Detected {
                        target_id: 1,
                        pos: Vec3::new(10.0 * drone_id as f32, 5.0, 2.0),
                    }),
                })
                .await
                .unwrap();
            tx_clone
                .send(SourcedEvent {
                    drone_id: drone_id,
                    event: MissionEvent::Camera(Detected {
                        target_id: 1,
                        pos: Vec3::new(12.0 * drone_id as f32, 5.0, 2.0),
                    }),
                })
                .await
                .unwrap();
        });
        drone_tasks.push(task);
    }

    drop(tx);

    for task in drone_tasks {
        task.await.unwrap();
    }
    coordinator_task.await.unwrap();
}
