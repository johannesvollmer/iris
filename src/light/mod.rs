use crate::film::spectrum::Spectrum;
use crate::geometry::HitInfo;
use crate::math::*;
use crate::scene::Scene;

pub mod emitter;
// pub mod point;

pub struct Visibility {
    hit_point: Point3f,
    light_point: Point3f,
    normal: Normal3f,
    hit_err: Vec3f,
    time: Float,
}

impl Visibility {
    pub fn new(hit: &HitInfo, light_point: Point3f) -> Self {
        Self {
            hit_point: hit.gg.point,
            hit_err: hit.gg.point_error,
            light_point,
            normal: hit.gg.ng,
            time: hit.gg.time,
        }
    }

    pub fn visible(&self, scene: &Scene) -> bool {
        let ray = Ray::spawn_to(
            self.hit_point,
            self.light_point,
            self.hit_err,
            self.normal,
            self.time,
        );
        scene.intersect(&ray).is_none()
    }
}

pub trait Light {
    fn sample(&self, hit: &HitInfo, samples: (f32, f32)) -> (Spectrum, Vec3f, Float, Visibility);
    fn pdf(&self, p: Point3f, wi: Vec3f, time: Float) -> Float;
    fn is_delta(&self) -> bool;
}
