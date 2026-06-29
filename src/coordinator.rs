use crate::agents::{Agent, MissionEntry};
use crate::event::MissionEvent;
use tokio::sync::mpsc;

pub struct Coordinator {
    agents: Vec<Box<dyn Agent + Send + Sync>>,
    sender: mpsc::Sender<MissionEvent>,
}

impl Coordinator {
    pub fn new(sender: mpsc::Sender<MissionEvent>) -> Self {
        Coordinator {
            agents: Vec::new(),
            sender,
        }
    }

    pub fn add_agent(&mut self, agent: Box<dyn Agent + Send + Sync>) {
        self.agents.push(agent);
    }

    pub async fn run(&mut self, mut receiver: mpsc::Receiver<MissionEvent>) {
        while let Some(event) = receiver.recv().await {
            for agent in &mut self.agents {
                let entry = agent.act(&event); // pass coordinator if needed
                match &entry.event {
                    MissionEvent::Alert(msg) => {
                        println!("*** ALERT from Agent {}: {} ***", entry.agent_id, msg)
                    }
                    MissionEvent::Radar(msg) => {
                        println!("Radar from Agent {}: {}", entry.agent_id, msg)
                    }
                    MissionEvent::Camera(msg) => {
                        println!("Camera from Agent {}: {}", entry.agent_id, msg)
                    }
                    MissionEvent::Idle => println!("Agent {} idle", entry.agent_id),
                }
            }
        }
    }

    pub fn dispatch(&self, event: MissionEvent) {
        let sender = self.sender.clone();
        tokio::spawn(async move {
            if let Err(e) = sender.send(event).await {
                eprintln!("Failed to dispatch event: {}", e);
            }
        });
    }
}
