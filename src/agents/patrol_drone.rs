use std::time::{Duration, Instant};

use crate::{
    agents::{Agent, MissionEntry},
    event::MissionEvent,
    transform::{DecisionModule, decision::CombinedTargetDecisionModule},
};

pub struct PatrolDrone {
    pub id: u32,
    pub radar_count: u32,
    pub radar_count_last_reset: Instant,
    pub target_detected_from_camera: Target,
    pub target_detected_from_radar: Target,
}

pub struct Target {
    pub target_dist: u32,
    pub last_seen: Instant,
}

impl Agent for PatrolDrone {
    fn act(&mut self, event: &MissionEvent) -> MissionEntry {
        let decision_module = CombinedTargetDecisionModule;
        match event {
            MissionEvent::Radar(target_dist) => {
                self.radar_count += 1;
                self.target_detected_from_radar = Target {
                    target_dist: *target_dist,
                    last_seen: Instant::now(),
                };

                let return_event;
                // Reset window every 10 seconds
                if self.radar_count_last_reset.elapsed() > Duration::from_secs(10) {
                    self.radar_count = 1;
                    self.radar_count_last_reset = Instant::now();
                }

                if self.radar_count >= 3 {
                    return_event = MissionEvent::Alert(
                        *target_dist,
                        // "Multiple radar hits in 10s window, Alert by agent {} sent to Coordinator",
                    )
                } else {
                    return_event = decision_module.decide(vec![
                        &self.target_detected_from_camera,
                        &self.target_detected_from_radar,
                    ]);
                }
                MissionEntry::new(self.id, return_event)
            }

            MissionEvent::Camera(target_dist) => {
                self.target_detected_from_camera = Target {
                    target_dist: *target_dist,
                    last_seen: Instant::now(),
                };
                MissionEntry::new(
                    self.id,
                    decision_module.decide(vec![
                        &self.target_detected_from_camera,
                        &self.target_detected_from_radar,
                    ]),
                )
            }

            _ => MissionEntry::new(self.id, event.clone()),
        }
    }

    fn id(&self) -> u32 {
        self.id
    }
}
