use crate::agents::{Agent, MissionEntry};

pub struct PatrolDrone {
    pub id: u32,
    pub radar_count: u32,
}

impl Agent for PatrolDrone {
    fn act<'a>(&mut self, fused_input: &'a str) -> MissionEntry {
        let msg = if fused_input.contains("Radar") {
            self.radar_count += 1;
            if self.radar_count >= 3 {
                self.radar_count = 0;
                format!("PatrolDrone {} ALERT: multiple radar detections!", self.id)
            } else {
                format!("PatrolDrone {} responding to {}", self.id, fused_input)
            }
        } else {
            format!("PatrolDrone {} idle", self.id)
        };

        MissionEntry::new(self.id, msg)
    }
    
    fn id(&self) -> u32 {
        self.id
    }
}

