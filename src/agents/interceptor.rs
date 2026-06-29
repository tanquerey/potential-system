use crate::{agents::MissionEntry, coordinator::Coordinator, event::MissionEvent};
use std::time::{Duration, Instant};

pub struct Interceptor {
    pub id: u32,
    pub radar_count: u32,
    pub last_reset: Instant,
}

impl Interceptor {
    pub fn new(id: u32) -> Self {
        Interceptor {
            id,
            radar_count: 0,
            last_reset: Instant::now(),
        }
    }

    pub fn act(&mut self, event: &MissionEvent, coordinator: &Coordinator) -> MissionEntry {
        match event {
            MissionEvent::Radar(_msg) => {
                self.radar_count += 1;

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
                    // Dispatch alert back into system
                    coordinator.dispatch(alert);
                }

                MissionEntry::new(self.id, MissionEvent::Radar("Radar ping".into()))
            }
            _ => MissionEntry::new(self.id, event.clone()),
        }
    }
}
