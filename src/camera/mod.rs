use crate::film::Film;
use crate::math::{Float, Point2f, Ray, RayDifferential, RayDifferentialInfo, Transform};
use std::ops::Range;

pub mod orthographic;

#[derive(Copy, Clone)]
pub struct CameraSample {
    pub film: Point2f,
    pub lens: Point2f,
    pub time: Float,
}

pub trait Camera {
    fn get_film(&self) -> &Film;
    fn camera_to_world(&self) -> &Transform;
    fn shutter_open(&self) -> Range<Float>;

    fn generate_ray(&self, sample: &CameraSample) -> Option<(Ray, Float)>;

    fn generate_ray_differential(&self, sample: &CameraSample) -> Option<(RayDifferential, Float)> {
        if let Some((ray, wt)) = self.generate_ray(sample) {
            let mut sample_shifted = *sample;
            sample_shifted.film.x += 1.0;

            let rx = self.generate_ray(&sample_shifted).map(|(rx, _)| rx)?;

            sample_shifted.film.x -= 1.0;
            sample_shifted.film.y += 1.0;

            let ry = self.generate_ray(&sample_shifted).map(|(ry, _)| ry)?;

            let rd = RayDifferential {
                ray,
                info: Some(RayDifferentialInfo {
                    rx_origin: rx.o,
                    rx_direction: rx.d,
                    ry_origin: ry.o,
                    ry_direction: ry.d,
                }),
            };

            Some((rd, wt))
        } else {
            None
        }
    }
}
