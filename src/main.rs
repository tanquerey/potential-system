mod agents;
mod coordinator;
mod fusion;
mod sensors;

use agents::{spawn_analyzer, spawn_interceptor, spawn_patrol_drone};
use coordinator::Coordinator;
use fusion::SimpleFusion;
use sensors::{CameraSensor, RadarSensor};
use tokio::sync::broadcast;

#[tokio::main]
async fn main() {
    // broadcast channel
    let (tx, _rx) = broadcast::channel::<String>(10);
    
    // Spawn agents with their own receivers
    spawn_patrol_drone(tx.subscribe());
    spawn_interceptor(tx.subscribe());
    spawn_analyzer(tx.subscribe());
    
    // Coordinator runs with sensors + fusion
    let system = Coordinator::new(
        vec![Box::new(RadarSensor), Box::new(CameraSensor)],
        SimpleFusion,
        tx,
    );
    system.run().await;
}
