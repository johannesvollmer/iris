use crate::geometry::HitInfo;
use crate::film::spectrum::Spectrum;
use crate::scene::Scene;
use crate::math::*;

pub mod emitter;
// pub mod point;

pub struct Visibility {
    origin: Point3f,
    point: Point3f,
    normal: Normal3f,
    err: Vec3f,
    time: Float,
}

impl Visibility {
    pub fn new(hit: &HitInfo, point: Point3f) -> Self {
        let gh = &hit.geometry_hit_info;
        Self {
            origin: gh.point,
            point,
            normal: gh.ng,
            err: gh.point_error,
            time: gh.time,
        }
    }

    pub fn occluded(&self, scene: &Scene) -> bool {
        let ray = Ray::spawn_to(self.origin, self.point, self.err, self.normal, self.time);
        scene.intersect(&ray).is_some()
    }
}

pub trait Light {
    fn sample(&self, hit: &HitInfo, samples: (f32, f32)) -> (Spectrum, Vec3f, Float, Visibility);
    fn pdf(&self, p: Point3f, wi: Vec3f, time: Float) -> Float;
    fn is_delta(&self) -> bool;
}