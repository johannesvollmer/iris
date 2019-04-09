use crate::film::spectrum::Spectrum;
use crate::geometry::Hit;
use crate::geometry::Interaction;
use crate::geometry::AABB;
use crate::math::*;
use crate::scene::Scene;

pub mod emitter;

pub mod diffuse_area;
pub mod point;
pub mod spot;

#[derive(Copy, Clone, PartialEq, Eq)]
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

pub trait Light: AABB + Hit {
    fn sample_incoming(
        &self,
        int: &Interaction,
        samples: (Float, Float),
    ) -> (Spectrum, Vec3f, Float);
    fn power(&self) -> Spectrum;

    fn radiance(&self, _int: &Interaction, _w: Vec3f) -> Spectrum {
        unimplemented!()
    }
    fn pdf_incoming(&self, _int: &Interaction, _wi: Vec3f) -> Float {
        unimplemented!()
    }
}
