use crate::agents::Agent;
use crate::agents::mission_entry::MissionEntry;

/// Analyzer agent: processes fused inputs and logs them
pub struct Analyzer {
    pub id: u32,
}

impl Agent for Analyzer {
    fn act<'a>(&mut self, fused_input: &'a str) -> MissionEntry {
        let msg = if fused_input.contains("Camera") {
            format!("Analyzer {} processed camera input: {}", self.id, fused_input)
        } else if fused_input.contains("Radar") {
            format!("Analyzer {} processed radar input: {}", self.id, fused_input)
        } else {
            format!("Analyzer {} idle, no relevant input", self.id)
        };

        MissionEntry::new(self.id, msg)
    }

    fn id(&self) -> u32 {
        self.id
    }
}
