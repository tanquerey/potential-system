use std::{
    collections::HashSet,
    sync::{Arc, Mutex},
};

use crate::{
    agents::{Agent, MissionEntry, patrol_drone::EngineError},
    event::MissionEvent,
};

pub struct Interceptor {
    pub id: u8,
    pub claimed: ClaimedTargets,
}

pub type ClaimedTargets = Arc<Mutex<HashSet<u8>>>; // target id -> claimed or not

impl Interceptor {
    pub fn new(id: u8, claimed: ClaimedTargets) -> Self {
        Interceptor { id, claimed }
    }
    pub fn try_claim(&self, target_id: u8) -> bool {
        let mut set = self.claimed.lock().unwrap();
        if set.contains(&target_id) {
            false // someone already claimed it
        } else {
            set.insert(target_id);
            true // we claimed it
        }
    }
}
impl Agent for Interceptor {
    fn act(&mut self, event: &MissionEvent) -> Result<MissionEntry, EngineError> {
        match event {
            MissionEvent::Intercept(target) => {
                println!(
                    "Interceptor {} received Intercept cmd for target at : {}",
                    self.id, target.pos
                );
                let response =
                    MissionEvent::Idle(format!("Interceptor {} engaged target!", self.id));
                Ok(MissionEntry::new(self.id, response))
            }
            _ => Ok(MissionEntry::new(self.id, event.clone())),
        }
    }

    fn id(&self) -> u8 {
        self.id
    }
}
