use std::time::{Duration, Instant};

use crate::{
    agents::{Agent, MissionEntry},
    event::MissionEvent,
};

pub struct PatrolDrone {
    pub id: u32,
    pub radar_count: u32,
    pub last_reset: Instant,
}

impl Agent for PatrolDrone {
    fn act(&mut self, event: &MissionEvent) -> MissionEntry {
        match event {
            MissionEvent::Radar(_msg) => {
                self.radar_count += 1;
                let mut system_event = MissionEvent::Radar("Radar ping".into());
                // Reset window every 10 seconds
                if self.last_reset.elapsed() > Duration::from_secs(10) {
                    self.radar_count = 1;
                    self.last_reset = Instant::now();
                }

                if self.radar_count >= 3 {
                    // Interceptor decides this is an Alert
                    let alert = MissionEvent::Alert(format!(
                        "Interceptor {} escalated: multiple radar hits in 10s window!",
                        self.id
                    ));
                    system_event = alert;
                }

                MissionEntry::new(self.id, system_event)
            }
            _ => MissionEntry::new(self.id, event.clone()),
        }
    }

    fn id(&self) -> u32 {
        self.id
    }
}
