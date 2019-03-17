use crate::camera::CameraSample;
use std::ops::Range;
use std::sync::Arc;
use crate::camera::Camera;
use crate::film::Film;
use crate::math::*;

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
    film: Arc<Film>
}

impl OrthographicCamera {
    fn new(camera_to_world: Transform, screen_window: Bounds2f, lens_radius: Float, focal_distance: Float, film: Arc<Film>) -> Self {
        let camera_to_screen = Transform::orthographic(0.0, 1.0);

        let screen_to_raster = Transform::scale(film.resolution().x, film.resolution().y, 1.0)
                                * Transform::scale(1.0 / (screen_window.max.x - screen_window.min.x), 1 / (screen_window.max.y - screen_window.min.y), 1.0)
                                * Transform::translate(Vec3f::new(-screen_window.min.x, -screen_window.max.y, 0.0));

        let raster_to_screen = screen_to_raster.inverse();

        let raster_to_camera = camera_to_screen.inverse() * raster_to_screen;

        let dx_camera = raster_to_camera.apply(Vec3f::new(1.0, 0.0, 0.0));
        let dy_camera = raster_to_camera.apply(Vec3f::new(0.0, 1.0, 0.0));

        Self {
            camera_to_screen,
            raster_to_camera,
            screen_to_raster,
            raster_to_screen,
            camera_to_world,
            dx_camera,
            dy_camera,
            lens_radius,
            focal_distance,
            film,
        }
    }
}

impl Camera for OrthographicCamera {
    fn shutter_open(&self) -> Range<Float> {
        self.shutter_open
    }

    fn get_film(&self) -> &Film {
        &*self.film
    }

    fn camera_to_world(&self) -> &Transform {
        &self.camera_to_world
    }

    fn generate_ray(&self, camera_sample: &CameraSample) -> Option<(Ray, Float)> {
        None
    }
}