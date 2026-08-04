use crate::agents::{Agent, AgentType, Interceptor};
use crate::event::MissionEvent;
use flux_perception::Engine;
use tokio::sync::mpsc;

pub struct Coordinator {
    agents: Vec<AgentType>,
}

impl Coordinator {
    pub fn new() -> Self {
        Coordinator { agents: Vec::new() }
    }

    pub fn add_agent(&mut self, agent: AgentType) {
        self.agents.push(agent);
    }

    pub async fn run(
        &mut self,
        mut receiver: mpsc::Receiver<MissionEvent>,
        mut engine: &mut Engine,
    ) {
        let mut interceptor = Interceptor { id: 3 };

        while let Some(event) = receiver.recv().await {
            let mut entries = Vec::new();
            for agent in &mut self.agents {
                match agent.act(&event, &mut engine) {
                    Ok(entry) => entries.push(entry),
                    Err(e) => eprintln!("Agent {} failed to act: {:?}", agent.id(), e),
                }
            }

            for entry in entries {
                match &entry.event {
                    MissionEvent::Alert(target_dist) | MissionEvent::Intercept(target_dist) => {
                        println!(
                            "ALERT from Agent {}: Intercepting target at distance {}",
                            entry.agent_id, target_dist
                        );
                        if let Err(e) =
                            interceptor.act(&MissionEvent::Intercept(*target_dist), &mut engine)
                        {
                            eprintln!("Interceptor failed: {:?}", e);
                        }
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
                    MissionEvent::Idle(msg) => {
                        println!("Agent {} is Idle with msg:{}", entry.agent_id, msg)
                    }
                }
            }
        }
    }
}
