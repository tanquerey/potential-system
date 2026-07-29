use std::time::{SystemTime, UNIX_EPOCH};

use flux_confidence::Confidence;
use flux_perception::Engine;

use crate::{
    agents::{Agent, MissionEntry},
    event::MissionEvent,
    transform::{DecisionModule, decision::SensorAwareDecision},
};

pub struct PatrolDrone {
    pub id: u32,
    pub camera_confidence: Confidence,
    pub radar_confidence: Confidence,
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
                
                // println!("Radar confidence Before :{} with agreement :{}", self.radar_confidence.value(), engine.agreement());
                self.radar_confidence.update(engine.agreement(), 1.0); 
                // println!("Radar confidence After :{} with agreement :{}", self.radar_confidence.value(), engine.agreement());
                self.radar_confidence.decay();
                engine.update(1, *target_dist, self.radar_confidence.value(), now_u64());

                return MissionEntry::new(1, sensor_aware_module.decide(engine));
            }

            MissionEvent::Camera(target_dist) => {
                
                // println!("Camera confidence Before :{} with agreement :{}", self.camera_confidence.value(), engine.agreement());
                self.camera_confidence.update(engine.agreement(), 1.0);
                // println!("Camera confidence After :{} with agreement :{}", self.camera_confidence.value(), engine.agreement());

                engine.update(2, *target_dist, self.camera_confidence.value(), now_u64());

                return MissionEntry::new(1, sensor_aware_module.decide(engine));
            }

            _ => MissionEntry::new(self.id, event.clone()),
        }
    }

    fn id(&self) -> u32 {
        self.id
    }
}
