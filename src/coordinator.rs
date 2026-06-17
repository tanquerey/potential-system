use tokio::sync::broadcast;
use tokio::task;
use std::sync::{Arc, Mutex};
use crate::fusion::FusedMessage;
use crate::agents::{Agent, MissionEntry};

pub struct Coordinator {
    agents: Vec<Box<dyn Agent>>,
    mission_log: Arc<Mutex<Vec<MissionEntry>>>,
    idle_timeout: std::time::Duration,
}

impl Coordinator {
    pub fn new(idle_timeout_secs: u64) -> Self {
        Coordinator {
            agents: Vec::new(),
            mission_log: Arc::new(Mutex::new(Vec::new())),
                        idle_timeout: std::time::Duration::from_secs(idle_timeout_secs),
        }
    }

    pub fn add_agent(&mut self, agent: Box<dyn Agent>) {
        self.agents.push(agent);
    }

    pub fn mission_log(&self) -> Arc<Mutex<Vec<MissionEntry>>> {
        Arc::clone(&self.mission_log)
    }

    pub fn into_parts(self) -> (Vec<Box<dyn Agent>>, Arc<Mutex<Vec<MissionEntry>>>) {
        (self.agents, self.mission_log)
    }

pub fn idle_timeout(&self) -> std::time::Duration {
        self.idle_timeout
    }
    
    pub fn spawn(
        mut radar_rx: broadcast::Receiver<FusedMessage>,
        mut camera_rx: broadcast::Receiver<FusedMessage>,
        mut agents: Vec<Box<dyn Agent>>,
        mission_log: Arc<Mutex<Vec<MissionEntry>>>,
        idle_timeout: std::time::Duration,
    ) {
        task::spawn(async move {
            loop {
                tokio::select! {
                    Ok(msg) = radar_rx.recv() => {
                        let fused_str = format!("Radar {:?}", msg);
                        for agent in agents.iter_mut() {
                            mission_log.lock().unwrap().push(agent.act(&fused_str));
                        }
                    }
                    Ok(msg) = camera_rx.recv() => {
                        let fused_str = format!("Camera {:?}", msg);
                        for agent in agents.iter_mut() {
                            mission_log.lock().unwrap().push(agent.act(&fused_str));
                        }
                    }
                    _ = tokio::time::sleep(idle_timeout) => {
                        let idle_entry = MissionEntry::new(0, format!("Coordinator idle for {:?}s", idle_timeout.as_secs()));
                        mission_log.lock().unwrap().push(idle_entry);
                    }
                }
            }
        });
    }


}
