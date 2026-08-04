use flux_perception::Engine;

use crate::{
    agents::{Agent, Interceptor, MissionEntry, PatrolDrone, patrol_drone::EngineError},
    event::MissionEvent,
};

pub enum AgentType {
    Patrol(PatrolDrone),
    Interceptor(Interceptor),
}

impl Agent for AgentType {
    fn act(
        &mut self,
        event: &MissionEvent,
        engine: &mut Engine,
    ) -> Result<MissionEntry, EngineError> {
        match self {
            AgentType::Patrol(p) => p.act(event, engine),
            AgentType::Interceptor(i) => i.act(event, engine),
        }
    }

    fn id(&self) -> u32 {
        return 0;
    }
}
