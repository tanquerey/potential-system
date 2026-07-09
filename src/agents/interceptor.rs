use crate::{
    agents::{Agent, MissionEntry},
    coordinator::Coordinator,
    event::MissionEvent,
};
use std::time::{Duration, Instant};

pub struct Interceptor {
    pub id: u32,
}

impl Interceptor {
    pub fn new(id: u32) -> Self {
        Interceptor { id }
    }
}
impl Agent for Interceptor {
    fn act(&mut self, event: &MissionEvent) -> MissionEntry {
        match event {
            MissionEvent::Command(cmd) => {
                let msg = format!("Interceptor {} received command: {}", self.id, cmd);

                // Example: escalate if command is "Engage"
                if cmd == "Engage" {
                    let response =
                        MissionEvent::Idle(format!("Interceptor {} engaged target!", self.id));
                    MissionEntry::new(self.id, response)
                } else {
                    MissionEntry::new(self.id, event.clone())
                }
            }
            _ => MissionEntry::new(self.id, event.clone()),
        }
    }

    fn id(&self) -> u32 {
        self.id
    }
}
