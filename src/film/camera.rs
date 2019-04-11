#![allow(dead_code)]

use crate::film::Film;
use crate::math::*;

#[derive(Copy, Clone)]
pub struct CameraSample {
    pub film: Point2f,
    pub lens: Point2f,
    pub time: Float,
}

pub struct PerspectiveCamera {
    raster_to_camera: Transform,
    raster_to_screen: Transform,
    camera_to_world: Transform,
}

impl PerspectiveCamera {
    pub fn new(camera_to_world: Transform, fov_deg: Float, film: &Film) -> Self {
        let aspect = film.full_resolution.x as Float / film.full_resolution.y as Float;
        let screen = if aspect >= 1.0 {
            Bounds2f::new(Point2f::new(-aspect, -1.0), Point2f::new(aspect, 1.0))
        } else {
            Bounds2f::new(
                Point2f::new(-1.0, -1.0 / aspect),
                Point2f::new(1.0, 1.0 / aspect),
            )
        };

        let screen_to_raster =
            Transform::scale(
                film.full_resolution.x as Float,
                film.full_resolution.y as Float,
                1.0,
            ) * Transform::scale(
                1.0 / (screen.max.x - screen.min.x),
                1.0 / (screen.min.y - screen.max.y),
                1.0,
            ) * Transform::translate(Vec3f::new(-screen.min.x, -screen.max.y, 0.0));

        let raster_to_screen = screen_to_raster.inverse();

        let camera_to_screen = Transform::perspective(fov_deg, 1e-2, 1000.0);
        let raster_to_camera = camera_to_screen.inverse() * raster_to_screen;

        Self {
            camera_to_world,
            raster_to_camera,
            raster_to_screen,
        }
    }

    pub fn generate_ray(&self, sample: &CameraSample) -> Option<(Ray, Float)> {
        let p_film = Point3f::new(sample.film.x, sample.film.y, 0.0);
        let p_camera = self.raster_to_camera.apply_point(p_film);

        let mut ray = Ray::new(Point3f::default(), p_camera.to_vec().normalized());

        // TODO: DoF
        ray.o = self.camera_to_world.apply_point(ray.o);
        ray.d = self.camera_to_world.apply(ray.d);

        Some((ray, 1.0))
    }
}
