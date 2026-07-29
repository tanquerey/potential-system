use flux_perception::{Engine, FusedSignal};

use crate::{event::MissionEvent, transform::DecisionModule};

pub struct SensorAwareDecision;

impl DecisionModule for SensorAwareDecision {
    fn decide(&self, engine: &Engine) -> MissionEvent {
        let fs: FusedSignal = engine.read();

        println!(
            "FLUX Perception value {}, source_count={}, confidence={}, variance={})",
                fs.value, fs.source_count, fs.confidence, fs.variance
        );
        // Decide based on fused confidence
        if fs.confidence >= 0.5 {
            MissionEvent::Intercept(fs.value)
        } else {
            MissionEvent::Idle(format!(
                "Low confidence (value={}, source_count={}, confidence={}, variance={})",
                fs.value, fs.source_count, fs.confidence, fs.variance
            ))
        }
    }
}
