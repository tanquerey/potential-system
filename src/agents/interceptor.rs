use flux_perception::Engine;

use crate::{
    agents::{Agent, MissionEntry, patrol_drone::EngineError},
    event::MissionEvent,
};

pub struct Interceptor {
    pub id: u32,
}

impl Interceptor {
    pub fn new(id: u32) -> Self {
        Interceptor { id }
    }
}
impl Agent for Interceptor {
    fn act(
        &mut self,
        event: &MissionEvent,
        _engine: &mut Engine,
    ) -> Result<MissionEntry, EngineError> {
        match event {
            MissionEvent::Intercept(target) => {
                println!(
                    "Interceptor {} received Intercept cmd for target at : {}",
                    self.id, target.dist
                );
                let response =
                    MissionEvent::Idle(format!("Interceptor {} engaged target!", self.id));
                Ok(MissionEntry::new(self.id, response))
            }
            _ => Ok(MissionEntry::new(self.id, event.clone())),
        }
    }

    fn id(&self) -> u32 {
        self.id
    }
}
