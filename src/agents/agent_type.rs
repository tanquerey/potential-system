use crate::{
    agents::{Agent, Interceptor, MissionEntry, PatrolDrone, patrol_drone::EngineError},
    event::MissionEvent,
};

pub enum AgentType {
    Patrol(PatrolDrone),
    Interceptor(Interceptor),
}

impl Agent for AgentType {
    fn act(&mut self, event: &MissionEvent) -> Result<MissionEntry, EngineError> {
        match self {
            AgentType::Patrol(p) => p.act(event),
            AgentType::Interceptor(i) => i.act(event),
        }
    }

    fn id(&self) -> u32 {
        match self {
            AgentType::Patrol(p) => p.id(),
            AgentType::Interceptor(i) => i.id(),
        }
    }
}
