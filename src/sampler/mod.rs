use crate::camera::CameraSample;
use crate::math::*;

pub mod uniform;

pub trait Sampler {
    fn get_bounds(&self) -> Bounds2i;
    fn start_pixel(&mut self, pixel: Point2i);
    fn clone_seed(&self, seed: u64) -> Box<dyn Sampler>;
    fn next_sample(&mut self) -> Option<f32>;
    fn samples_per_pixel(&self) -> u32;

    fn get_1d(&mut self) -> Float;
    fn get_2d(&mut self) -> Vec2f;

    fn get_camera_sample(&mut self, raster: Point2i) -> CameraSample {
        let lens_vec = self.get_2d();

        CameraSample {
            film: Point2f::from(raster) + self.get_2d(),
            time: self.get_1d(),
            lens: Point2f::new(lens_vec.x, lens_vec.y),
        }
    }
}
