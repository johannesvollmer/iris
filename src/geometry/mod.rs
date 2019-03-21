use crate::math::*;

pub mod primitive;
pub mod receiver;
pub mod sphere;

pub struct HitInfo {
    pub point: Point3f,
    pub normal: Vec3f,
    pub time: Float,
}

pub trait Geometry {
    fn aabb(&self) -> Bounds3f;
    fn intersect(&self, ray: &mut Ray) -> Option<HitInfo>;
}
