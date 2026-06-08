use tokio::sync::{broadcast};
use tokio::task;

pub fn spawn_patrol_drone(mut rx: broadcast::Receiver<String>) {
    task::spawn(async move {
        while let Ok(data) = rx.recv().await {
            println!("PatrolDrone responding to {}", data);
        }
    });
}
