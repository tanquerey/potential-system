use crate::{agents::patrol_drone::Target, event::MissionEvent, transform::DecisionModule};

pub struct CombinedTargetDecisionModule;

impl DecisionModule for CombinedTargetDecisionModule {
    fn decide(&self, targets: Vec<&Target>) -> MissionEvent {
        let target_dist = targets[0].target_dist;

        if targets.iter().all(|t| t.target_dist == target_dist) {
            MissionEvent::Intercept(target_dist)
        } else {
            MissionEvent::Idle("".into())
        }
    }
}
