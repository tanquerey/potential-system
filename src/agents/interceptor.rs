use crate::agents::{Agent, MissionEntry};

pub struct Interceptor {
    pub id: u32,
    pub last_radar_count: u32,
}

impl Agent for Interceptor {
    fn act<'a>(&mut self, fused_input: &'a str) -> MissionEntry {
        let mut current_count = self.last_radar_count;

        if fused_input.contains("Radar") {
            current_count += 1;
        }

        let msg = if current_count > self.last_radar_count {
            format!("Interceptor {} detected new radar activity!", self.id)
        } else {
            format!("Interceptor {} no change in radar count", self.id)
        };

        self.last_radar_count = current_count;
        MissionEntry::new(self.id, msg)
    }

    fn id(&self) -> u32 {
        self.id
    }
}
