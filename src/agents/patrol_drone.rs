use std::time::{SystemTime, UNIX_EPOCH};

use flux_confidence::Confidence;
use flux_perception::Engine;
use tokio::time::Instant;

use crate::{
    agents::{Agent, MissionEntry},
    event::{
        CAMERA,
        MissionEvent::{self, Alert},
        RADAR,
    },
};

pub struct PatrolDrone {
    pub id: u32,
    pub engine: Engine,
    pub radar_confidence: Confidence,
    pub camera_confidence: Confidence,
    pub last_seen: Instant,
}

fn now_u64() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("system time is before UNIX_EPOCH")
        .as_millis() as u64
}

#[derive(Debug, Clone)]
pub enum EngineError {
    UnknownOrInactiveSensor(u8),
}

fn safe_update(
    engine: &mut Engine,
    sensor_id: u8,
    value: f64,
    confidence: f64,
    now: u64,
) -> Result<(), EngineError> {
    match engine.find_sensor(sensor_id) {
        Some(s) if s.active => {
            engine.update(sensor_id, value, confidence, now);
            Ok(())
        }
        _ => Err(EngineError::UnknownOrInactiveSensor(sensor_id)),
    }
}

impl Agent for PatrolDrone {
    fn act(&mut self, event: &MissionEvent) -> Result<MissionEntry, EngineError> {
        self.last_seen = Instant::now();
        match event {
            MissionEvent::Radar(target) => {
                println!(
                    "Agent {} Radar confidence Before :{} with agreement :{} with age :{}",
                    self.id,
                    self.radar_confidence.value(),
                    self.engine.agreement(),
                    self.radar_confidence.age()
                );

                safe_update(
                    &mut self.engine,
                    RADAR.id,
                    target.dist,
                    self.radar_confidence.value(),
                    now_u64(),
                )?;
                self.radar_confidence.update(self.engine.agreement(), 1.0);
                println!(
                    "Agent {} Radar confidence After :{} with agreement :{} with age :{}",
                    self.id,
                    self.radar_confidence.value(),
                    self.engine.agreement(),
                    self.radar_confidence.age()
                );
                let tracking = target.clone().track(self.engine.read().value);

                Ok(MissionEntry::new(self.id, Alert(tracking)))
            }

            MissionEvent::Camera(target) => {
                // println!("Camera confidence Before :{} with agreement :{}", self.camera_confidence.value(), engine.agreement());

                safe_update(
                    &mut self.engine,
                    CAMERA.id,
                    target.dist,
                    self.camera_confidence.value(),
                    now_u64(),
                )?;
                self.camera_confidence.update(self.engine.agreement(), 1.0);
                // println!("Camera confidence After :{} with agreement :{}", self.camera_confidence.value(), engine.agreement());

                let tracking = target.clone().track(self.engine.read().value);
                Ok(MissionEntry::new(self.id, Alert(tracking)))
            }

            _ => Ok(MissionEntry::new(self.id, event.clone())),
        }
    }

    fn id(&self) -> u32 {
        self.id
    }
    
    fn last_seen(&self) -> Option<Instant> {
        Some(self.last_seen)
    }
}
