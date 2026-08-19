use flux_perception::Engine;
use std::time::Instant;

use crate::event::MissionEvent;


/// Wraps flux-perception fusion into a simple decision module
pub struct ConfidenceScoreModule {
    pub threshold: f64,   // minimum confidence to intercept
}

impl ConfidenceScoreModule {
    pub fn new(threshold: f64) -> Self {
        Self { threshold }
    }

    /// Fuse radar + camera readings using flux-perception
    pub fn decide(
        &self,
        radar_dist: u32,
        radar_conf: f32,
        camera_dist: u32,
        camera_conf: f32,
    ) -> MissionEvent {
        let now = Instant::now();

        // Create fusion engine with confidence threshold
        let mut engine = Engine::new(self.threshold);

        // Register sensors (id, weight, bias)
        engine.add_sensor(1, 0.6, 0.0); // radar: higher weight
        engine.add_sensor(2, 0.4, 0.0); // camera: lower weight

        // Feed readings
        engine.update(1, (radar_dist as f32).into(), radar_conf.into(), 1);
        engine.update(2, (camera_dist as f32).into(), camera_conf.into(), 1);

        // Read fused signal
        let fused = engine.read();

        // Decide based on fused confidence
        // if fused.confidence >= self.threshold {
            // MissionEvent::Intercept()
        // } else {
            MissionEvent::Idle(format!(
                "Low confidence (radar={}, camera={}, fused={:.2})",
                radar_conf, camera_conf, fused.confidence
            ))
        //}
    }
}
