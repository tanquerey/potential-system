mod coordinator;
mod fusion;
mod agents;

use tokio::sync::broadcast;
use crate::agents::{PatrolDrone, Interceptor, Analyzer};
use crate::coordinator::Coordinator;
use crate::fusion::FusedMessage;

#[tokio::main]
async fn main() {
    let (tx, rx) = broadcast::channel(16);

    let mut coordinator = Coordinator::new();
    coordinator.add_agent(Box::new(PatrolDrone { id: 1, radar_count: 0 }));
  
    let (agents, mission_log) = coordinator.into_parts();
    Coordinator::spawn(rx, agents, mission_log.clone());

    // simulate fused messages
    tx.send(FusedMessage::Radar("Ping".into())).unwrap();
    tx.send(FusedMessage::Camera("Target".into())).unwrap();

    tokio::time::sleep(std::time::Duration::from_secs(2)).await;

    // inspect mission log after run
    let log = mission_log.lock().unwrap();
    println!("Final mission log: {:?}", *log);
}
