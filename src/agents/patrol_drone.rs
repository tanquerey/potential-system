use std::{
    collections::HashMap,
    time::{Duration, SystemTime, UNIX_EPOCH},
};

use flux_confidence::Confidence;
use flux_perception::Engine;
use glam::Vec3;
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
    pub id: u8,
    pub current_pos: Vec3,
    pub engine: Engine,
    pub radar_confidence: Confidence,
    pub camera_confidence: Confidence,
    pub last_seen: Instant,
    pub known_targets: HashMap<u8, (Vec3, Instant)>,
    pub next_target_id: u8,
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
    current_pos: Vec3,
    value: Vec3,
    confidence: f64,
    now: u64,
) -> Result<(), EngineError> {
    match engine.find_sensor(sensor_id) {
        Some(s) if s.active => {
            engine.update(
                sensor_id,
                value.distance(current_pos) as f64,
                confidence,
                now,
            );
            Ok(())
        }
        _ => Err(EngineError::UnknownOrInactiveSensor(sensor_id)),
    }
}

impl PatrolDrone {
    fn resolve_target_id(&mut self, pos: Vec3, match_radius: f32, stale_after: Duration) -> u8 {
        let now = Instant::now();

        // prune anything not refreshed recently, before attempting a match
        self.known_targets
            .retain(|_, (_, last_seen)| now.duration_since(*last_seen) <= stale_after);

        for (&id, (known_pos, _)) in self.known_targets.iter() {
            if pos.distance(*known_pos) <= match_radius {
                self.known_targets.insert(id, (pos, now));
                return id;
            }
        }

        let id = self.next_target_id;
        self.next_target_id += 1;
        self.known_targets.insert(id, (pos, now));
        id
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
                    self.current_pos,
                    target.pos,
                    self.radar_confidence.value(),
                    now_u64(),
                )?;
                self.radar_confidence.update(self.engine.agreement(), 1.0);

                let target_id = self.resolve_target_id(target.pos, 5.0, Duration::from_mins(5));
                let tracking = target
                    .clone()
                    .track(target_id, self.engine.read().confidence);
                println!(
                    "Agent {} Radar confidence After :{} with confidence :{} with agreement: {} with value :{}",
                    self.id,
                    self.radar_confidence.value(),
                    self.engine.read().confidence,
                    self.engine.agreement(),
                    self.engine.read().value
                );
                Ok(MissionEntry::new(self.id, Alert(tracking)))
            }

            MissionEvent::Camera(target) => {
                println!(
                    "Agent {} Camera confidence Before :{} with agreement :{}",
                    self.id,
                    self.camera_confidence.value(),
                    self.engine.agreement()
                );

                safe_update(
                    &mut self.engine,
                    CAMERA.id,
                    self.current_pos,
                    target.pos,
                    self.camera_confidence.value(),
                    now_u64(),
                )?;
                self.camera_confidence.update(self.engine.agreement(), 1.0);
                println!(
                    "Agent {} Camera confidence After :{} with agreement :{}",
                    self.id,
                    self.camera_confidence.value(),
                    self.engine.agreement()
                );
                let target_id = self.resolve_target_id(target.pos, 5.0, Duration::from_mins(5)); // tune match_radius to taste
                let tracking = target.clone().track(target_id, self.engine.read().value);

                Ok(MissionEntry::new(self.id, Alert(tracking)))
            }

            _ => Ok(MissionEntry::new(self.id, event.clone())),
        }
    }

    fn id(&self) -> u8 {
        self.id
    }

    fn last_seen(&self) -> Option<Instant> {
        Some(self.last_seen)
    }
}
