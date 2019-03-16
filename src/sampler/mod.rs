use crate::math::*;

pub mod uniform;

pub trait Sampler {
    fn get_bounds(&self) -> Bounds2i;
    fn start_pixel(&mut self, pixel: Point2i);
    fn clone_seed(&self, seed: u64) -> Box<dyn Sampler>;
    fn next_sample(&mut self) -> Option<f32>;
}
