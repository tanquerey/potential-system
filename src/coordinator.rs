use crate::agents::{Agent, AgentType, Interceptor};
use crate::event::MissionEvent::{self, Intercept};
use crate::event::SourcedEvent;
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

    pub async fn run(&mut self, mut receiver: mpsc::Receiver<SourcedEvent>) {
        let mut interceptor = Interceptor { id: 3 };

        while let Some(sourced) = receiver.recv().await {
            let mut entries = Vec::new();
            for agent in &mut self.agents {
                if agent.id() == sourced.drone_id {
                    match agent.act(&sourced.event) {
                        Ok(entry) => entries.push(entry),
                        Err(e) => eprintln!("Agent {} failed to act: {:?}", agent.id(), e),
                    }
                }
            }

            for entry in entries {
                match &entry.event {
                    MissionEvent::Alert(tracking) => match tracking.clone().intercept() {
                        Some(intercepting) => {
                            if let Err(e) = interceptor.act(&Intercept(intercepting)) {
                                eprintln!("Interceptor failed: {:?}", e);
                            }
                        }
                        None => {
                            println!(
                                "ALERT from Agent {}: target {:?} not confident enough to intercept",
                                entry.agent_id, tracking
                            );
                        }
                    },
                    MissionEvent::Radar(msg) => {
                        println!("Radar from Agent {}: {:?}", entry.agent_id, msg)
                    }
                    MissionEvent::Camera(msg) => {
                        println!("Camera from Agent {}: {:?}", entry.agent_id, msg)
                    }
                    MissionEvent::Command(msg) => {
                        println!("Command executed by Agent {}: {}", entry.agent_id, msg)
                    }
                    MissionEvent::Idle(msg) => {
                        println!("Agent {} is Idle with msg:{}", entry.agent_id, msg)
                    }
                    MissionEvent::Intercept(intercepting) => {
                        println!("Agent {} intercepted {:?}", entry.agent_id, intercepting)
                    }
                }
            }
        }
    }
}
