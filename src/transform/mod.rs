pub mod confidence;
pub mod decision;

use flux_perception::Engine;

pub trait DecisionModule {
    fn read_fused_value(&self, engine: &Engine) -> f64;
}
