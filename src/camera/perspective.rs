#![allow(dead_code)]

use crate::camera::CameraSample;
use crate::camera::Camera;
use crate::math::*;
use crate::film::Film;
use std::sync::Arc;
use std::ops::Range;

pub struct PerspectiveCamera {
    raster_to_camera: Transform,
    raster_to_screen: Transform,
    camera_to_world: Transform,
    shutter_open: Range<Float>,
    film: Arc<Film>,
}

impl PerspectiveCamera {
    pub fn new(
        camera_to_world: Transform,
        fov: Float,
        film: Arc<Film>,
    ) -> Self {
        let aspect = (film.full_resolution.x as Float) / (film.full_resolution.y as Float);

        let screen_to_raster =
            Transform::scale(
                film.full_resolution.x as Float,
                film.full_resolution.y as Float,
                1.0,
            ) * Transform::translate(Vec3f::new(0.0, -1.0, 0.0));
        let raster_to_screen = screen_to_raster.inverse();

        let camera_to_screen = Transform::perspective(aspect, fov.to_radians(), 1e-2, 1000.0);
        let raster_to_camera = camera_to_screen.inverse() * raster_to_screen;

        dbg!(camera_to_world);
        dbg!(camera_to_world.apply(Vec3f::new(0.0, 1.0, 0.0)));

        let shutter_open = Range {
            start: 0.0,
            end: 1.0,
        };

        Self {
            camera_to_world,
            raster_to_camera,
            raster_to_screen,
            shutter_open,
            film,
        }
    }
}

impl Camera for PerspectiveCamera {
    fn shutter_open(&self) -> Range<Float> {
        self.shutter_open.clone()
    }

    fn get_film(&self) -> &Film {
        &*self.film
    }

    fn camera_to_world(&self) -> &Transform {
        &self.camera_to_world
    }

    fn generate_ray(&self, sample: &CameraSample) -> Option<(Ray, Float)> {
        let v_film = Vec3f::new(sample.film.x, sample.film.y, 0.0);
        let v_camera = self.raster_to_camera.apply(v_film);

        let mut ray = Ray::new(Point3f::default(), v_camera.normalized());

        // TODO: DoF
        ray.time = lerp(sample.time, self.shutter_open.start, self.shutter_open.end);
        ray.o = self.camera_to_world.apply_point(ray.o);
        ray.d = self.camera_to_world.apply(ray.d);
        dbg!(ray);
        panic!();

        Some((ray, 1.0))
    }
}