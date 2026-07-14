use crate::agents::{Agent, Interceptor};
use crate::event::MissionEvent;
use tokio::sync::mpsc;

pub struct Coordinator {
    agents: Vec<Box<dyn Agent + Send + Sync>>,
}

impl Coordinator {
    pub fn new() -> Self {
        Coordinator { agents: Vec::new() }
    }

    pub fn add_agent(&mut self, agent: Box<dyn Agent + Send + Sync>) {
        self.agents.push(agent);
    }

    pub async fn run(&mut self, mut receiver: mpsc::Receiver<MissionEvent>) {
        let mut interceptor = Interceptor { id: 3 };

        while let Some(event) = receiver.recv().await {
            let mut entries = Vec::new();
            for agent in &mut self.agents {
                entries.push(agent.act(&event));
            }

            for entry in entries {
                match &entry.event {
                    MissionEvent::Alert(target_dist) | MissionEvent::Intercept(target_dist) => {
                        println!(
                            "ALERT from Agent {}: Intercepting target at distance {}",
                            entry.agent_id, target_dist
                        );
                        interceptor.act(&MissionEvent::Intercept(*target_dist));
                    }
                    MissionEvent::Radar(msg) => {
                        println!("Radar from Agent {}: {}", entry.agent_id, msg)
                    }
                    MissionEvent::Camera(msg) => {
                        println!("Camera from Agent {}: {}", entry.agent_id, msg)
                    }
                    MissionEvent::Command(msg) => {
                        println!("Command executed by Agent {}: {}", entry.agent_id, msg)
                    }
                    MissionEvent::Idle(_msg) => println!("Agent {} idle", entry.agent_id),
                }
            }
        }
    }
}
