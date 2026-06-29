use crate::{
    agents::{Agent, MissionEntry},
    event::MissionEvent,
};

pub struct PatrolDrone {
    pub id: u32,
    pub radar_count: u32,
}

impl Agent for PatrolDrone {
    fn act(&mut self, event: &MissionEvent) -> MissionEntry {
        match event {
            MissionEvent::Radar(msg) => {
                // Example: if radar_count exceeds threshold, raise an alert
                if self.radar_count > 3 {
                    MissionEntry::new(
                        self.id,
                        MissionEvent::Alert(format!(
                            "PatrolDrone {} ALERT: multiple radar detections!",
                            self.id
                        )),
                    )
                } else {
                    MissionEntry::new(self.id, MissionEvent::Radar(msg.clone()))
                }
            }
            _ => MissionEntry::new(self.id, event.clone()),
        }
    }

    fn id(&self) -> u32 {
        self.id
    }
}
