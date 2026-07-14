use super::FusionModule;

pub struct ResilientFusion;

impl FusionModule for ResilientFusion {
    fn fuse(&self, inputs: Vec<String>) -> String {
        let mut clean_inputs = Vec::new();

        for input in inputs {
            // Filter out spoofed or noisy signals
            if input.contains("noise") || input.contains("spoof") {
                println!("ResilientFusion ignored: {}", input);
                continue;
            }
            clean_inputs.push(input);
        }

        if clean_inputs.is_empty() {
            "No valid sensor data".to_string()
        } else {
            clean_inputs.join(" + ")
        }
    }
}
