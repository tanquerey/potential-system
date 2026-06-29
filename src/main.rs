mod agents;
mod coordinator;
mod event;
mod fusion;
use crate::agents::PatrolDrone;
use crate::coordinator::Coordinator;
use crate::event::MissionEvent;
use tokio::sync::mpsc;

#[tokio::main]
async fn main() {
    // Channel for mission events
    let (tx, rx) = mpsc::channel(32);

    // Create coordinator with sender
    let mut coordinator = Coordinator::new(tx.clone());

    // Add a patrol drone agent
    let drone = PatrolDrone {
        id: 1,
        radar_count: 0,
    };
    coordinator.add_agent(Box::new(drone));

    // Spawn coordinator task
    let coordinator_task = tokio::spawn(async move {
        coordinator.run(rx).await;
    });

    // Dispatch some events
    tx.send(MissionEvent::Radar("Ping".into())).await.unwrap();
    tx.send(MissionEvent::Camera("Target acquired".into()))
        .await
        .unwrap();
    tx.send(MissionEvent::Idle).await.unwrap();
    tx.send(MissionEvent::Alert("Intruder detected".into()))
        .await
        .unwrap();

    // Wait for coordinator to finish (in real system, this would run indefinitely)
    coordinator_task.await.unwrap();
}
