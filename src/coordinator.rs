use crate::agents::Agent;
use crate::fusion::FusedMessage;
use std::sync::{Arc, Mutex};
use tokio::sync::broadcast;
use tokio::task;

/// Coordinator with heterogeneous agents and shared mission log
pub struct Coordinator {
    agents: Vec<Box<dyn Agent>>,
    mission_log: Arc<Mutex<Vec<String>>>,
}

impl Coordinator {
    pub fn new() -> Self {
        Coordinator {
            agents: Vec::new(),
            mission_log: Arc::new(Mutex::new(Vec::new())),
        }
    }

    pub fn add_agent(&mut self, agent: Box<dyn Agent>) {
        self.agents.push(agent);
    }

    pub fn mission_log(&self) -> Arc<Mutex<Vec<String>>> {
        Arc::clone(&self.mission_log)
    }

    pub fn spawn(
        mut rx: broadcast::Receiver<FusedMessage>,
        mut agents: Vec<Box<dyn Agent>>,
        mission_log: Arc<Mutex<Vec<String>>>,
    ) {
        task::spawn(async move {
            while let Ok(msg) = rx.recv().await {
                let fused_str = format!("{:?}", msg);

                for agent in agents.iter_mut() {
                    let output = agent.act(&fused_str);

                    // push into shared mission log
                    {
                        let mut log = mission_log.lock().unwrap();
                        log.push(output.clone());
                    }

                    println!("{}", output);
                }
            }
        });
    }
    pub fn into_parts(self) -> (Vec<Box<dyn Agent>>, Arc<Mutex<Vec<String>>>) {
        (self.agents, self.mission_log)
    }
}
