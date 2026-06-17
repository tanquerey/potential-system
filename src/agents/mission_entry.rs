use std::time::SystemTime;

// A structured log entry for agent actions
pub struct MissionEntry {
    pub agent_id: u32,
    pub timestamp: SystemTime,
    pub message: String,
}

impl MissionEntry {
    pub fn new(agent_id: u32, message: String) -> Self {
        MissionEntry {
            agent_id,
            timestamp: SystemTime::now(),
            message,
        }
    }
}
