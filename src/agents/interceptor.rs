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
            MissionEvent::Intercept(target_dist) => {
                println!("Interceptor {} received Intercept cmd for target at : {}", self.id, target_dist);
                    let response =
                        MissionEvent::Idle(format!("Interceptor {} engaged target!", self.id));
                    MissionEntry::new(self.id, response)
            
            }
            _ => MissionEntry::new(self.id, event.clone()),
        }
    }

    fn id(&self) -> u32 {
        self.id
    }
}
