pub mod simple;

pub use simple::SimpleFusion;
pub trait FusionModule {
    fn fuse(&self, inputs: Vec<String>) -> String;
}
