use crate::math::*;

pub trait Sampler {
    fn get_bounds(&self) -> Bounds2i;
    fn start_pixel(&self, point: Point2i);
    fn start_next_sample(&self) -> bool;
}
