use flux_perception::{Engine, FusedSignal};

use crate::transform::DecisionModule;

pub struct SensorAwareDecision;

impl DecisionModule for SensorAwareDecision {
    fn read_fused_value(&self, engine: &Engine) -> f64 {
        let fs: FusedSignal = engine.read();

        println!(
            "FLUX Perception value {}, source_count={}, confidence={}, variance={})",
            fs.value, fs.source_count, fs.confidence, fs.variance
        );
        fs.value
    }
}
