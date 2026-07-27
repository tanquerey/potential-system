use std::time::{Duration, Instant, SystemTime, UNIX_EPOCH};

use flux_perception::Engine;

use crate::{
    agents::{Agent, MissionEntry}, event::MissionEvent, transform::{DecisionModule, decision::{ SensorAwareDecision}},
};

pub struct PatrolDrone {
    pub id: u32,
    pub radar_count: u32,
    pub radar_count_last_reset: Instant
}

pub struct Target {
    pub sensor_id: u8,
    pub target_dist: f64,
    pub last_seen: Instant,
}

fn now_u64() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("system time is before UNIX_EPOCH")
        .as_millis() as u64
}

impl Agent for PatrolDrone {
    fn act(&mut self, event: &MissionEvent, engine: &mut Engine) -> MissionEntry {
        
        let sensor_aware_module = SensorAwareDecision;

        match event {
            MissionEvent::Radar(target_dist) => {

                engine.update(1, *target_dist, 1.0, now_u64());

                return MissionEntry::new(1, sensor_aware_module.decide(&engine));
            }

            MissionEvent::Camera(target_dist) => {
                
                engine.update(2, *target_dist, 1.0, now_u64());

                return MissionEntry::new(1, sensor_aware_module.decide(&engine));
            }

            _ => MissionEntry::new(self.id, event.clone()),
        }
    }

    fn id(&self) -> u32 {
        self.id
    }
}
