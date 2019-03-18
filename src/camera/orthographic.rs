use crate::camera::Camera;
use crate::camera::CameraSample;
use crate::film::Film;
use crate::math::*;
use std::ops::Range;
use std::sync::Arc;

pub struct OrthographicCamera {
    camera_to_screen: Transform,
    raster_to_camera: Transform,
    screen_to_raster: Transform,
    raster_to_screen: Transform,
    camera_to_world: Transform,
    dx_camera: Vec3f,
    dy_camera: Vec3f,
    shutter_open: Range<Float>,
    lens_radius: Float,
    focal_distance: Float,
    film: Arc<Film>,
}

impl OrthographicCamera {
    pub fn new(
        camera_to_world: Transform,
        screen_window: Bounds2f,
        lens_radius: Float,
        focal_distance: Float,
        film: Arc<Film>,
    ) -> Self {
        assert!(lens_radius == 0.0);
        let camera_to_screen = Transform::orthographic(0.0, 1.0);

        let screen_to_raster =
            Transform::scale(
                film.full_resolution.x as Float,
                film.full_resolution.y as Float,
                1.0,
            ) * Transform::scale(
                1.0 / (screen_window.max.x - screen_window.min.x),
                1.0 / (screen_window.max.y - screen_window.min.y),
                1.0,
            ) * Transform::translate(Vec3f::new(-screen_window.min.x, -screen_window.max.y, 0.0));

        let raster_to_screen = screen_to_raster.inverse();

        let raster_to_camera = camera_to_screen.inverse() * raster_to_screen;

        let dx_camera = raster_to_camera.apply(Vec3f::new(1.0, 0.0, 0.0));
        let dy_camera = raster_to_camera.apply(Vec3f::new(0.0, 1.0, 0.0));

        let shutter_open = std::ops::Range {
            start: 0.0,
            end: 0.0001,
        };

        Self {
            camera_to_screen,
            raster_to_camera,
            screen_to_raster,
            raster_to_screen,
            camera_to_world,
            dx_camera,
            dy_camera,
            shutter_open,
            lens_radius,
            focal_distance,
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
        // DoF
        ray.time = lerp(sample.time, self.shutter_open.start, self.shutter_open.end);
        ray.o = self.camera_to_world.apply(ray.o.into()).into();
        ray.d = self.camera_to_world.apply(ray.d);

        Some((ray, 1.0))
    }
}
