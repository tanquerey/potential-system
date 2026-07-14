use super::FusionModule;

pub struct SimpleFusion;

impl FusionModule for SimpleFusion {
    fn fuse(&self, inputs: Vec<String>) -> String {
        inputs.join(" + ")
    }
}
