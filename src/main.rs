mod agents;
mod coordinator;
mod fusion;
mod sensors;

use fusion::{ResilientFusion, FusedMessage};
use sensors::{RadarSensor, CameraSensor};
// use agents::{ spawn_interceptor};
use coordinator::Coordinator;
use tokio::sync::broadcast;

use crate::agents::{Analyzer, Interceptor, PatrolDrone};

#[tokio::main]
async fn main() {
    let (tx, _rx) = broadcast::channel::<FusedMessage>(10);

    PatrolDrone::spawn(tx.subscribe(), 1);
    Interceptor::spawn(tx.subscribe());
    Analyzer::spawn(tx.subscribe());

    let system = Coordinator::new(
        vec![Box::new(RadarSensor), Box::new(CameraSensor)],
        ResilientFusion,
        tx,
    );
    system.run().await;
}
