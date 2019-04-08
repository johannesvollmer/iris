use crate::film::spectrum::Spectrum;
use crate::geometry::Interaction;
use crate::math::*;
use crate::scene::Scene;

pub mod emitter;

pub mod area;
pub mod point;
pub mod spot;

#[derive(Debug, Copy, Clone)]
#[allow(dead_code)]
pub enum LightType {
    Point,
    Spot,
    Area,
}

pub struct Visibility {
    hit_point: Point3f,
    light_point: Point3f,
    normal: Normal3f,
    hit_err: Vec3f,
    time: Float,
}

impl Visibility {
    pub fn new(int: &Interaction, light_point: Point3f) -> Self {
        Self {
            hit_point: int.point,
            hit_err: int.point_error,
            light_point,
            normal: int.normal,
            time: int.time,
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
    fn sample(
        &self,
        world_point: Point3f,
        samples: (Float, Float),
    ) -> (Spectrum, LocalPoint3f, Float);

    fn power(&self) -> Spectrum;

    fn light_to_world(&self) -> &Transform;

    fn pdf(&self, _p: LocalPoint3f, _wi: LocalVec3f) -> Float {
        unimplemented!()
    }
}
