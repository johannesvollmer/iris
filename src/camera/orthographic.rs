#![allow(dead_code)]

use crate::camera::Camera;
use crate::camera::CameraSample;
use crate::film::Film;
use crate::math::*;
use std::ops::Range;
use std::sync::Arc;

pub struct OrthographicCamera {
    raster_to_camera: Transform,
    camera_to_world: Transform,
    shutter_open: Range<Float>,
    film: Arc<Film>,
}

impl OrthographicCamera {
    pub fn new(
        camera_to_world: Transform,
        film: Arc<Film>,
    ) -> Self {
        let camera_to_screen = Transform::orthographic(0.0, 1.0);

        let screen_to_raster =
            Transform::scale(
                film.full_resolution.x as Float,
                film.full_resolution.y as Float,
                1.0,
            ) * Transform::translate(Vec3f::new(0.0, -1.0, 0.0));

        let raster_to_screen = screen_to_raster.inverse();

        let raster_to_camera = camera_to_screen.inverse() * raster_to_screen;

        let shutter_open = std::ops::Range {
            start: 0.0,
            end: 0.0001,
        };

        Self {
            raster_to_camera,
            camera_to_world,
            shutter_open,
            film,
        }
    }
}

impl Camera for OrthographicCamera {
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

        let mut ray = Ray::new(v_camera.into(), Vec3f::new(0.0, 0.0, 1.0));

        // TODO: DoF
        ray.time = lerp(sample.time, self.shutter_open.start, self.shutter_open.end);
        ray.o = self.camera_to_world.apply(ray.o.into()).into();
        ray.d = self.camera_to_world.apply(ray.d);

        Some((ray, 1.0))
    }
}
