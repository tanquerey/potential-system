use std::time::SystemTime;

use crate::event::MissionEvent;

// A structured log entry for agent actions
pub struct MissionEntry {
    pub agent_id: u32,
    pub timestamp: SystemTime,
    pub event: MissionEvent,
}

impl MissionEntry {
    pub fn new(agent_id: u32, event: MissionEvent) -> Self {
        MissionEntry {
            agent_id,
            timestamp: SystemTime::now(),
            event,
        }
    }
}
