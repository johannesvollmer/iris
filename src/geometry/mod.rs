use crate::math::*;

pub mod primitive;
pub mod receiver;
pub mod sphere;

pub struct SurfaceInteraction {
    point: Point3f,
    time: Float,
    normal: Vec3f,
}

pub trait Geometry {
    fn aabb(&self) -> Bounds3f;
    fn intersect(&self, ray: &mut Ray) -> Option<SurfaceInteraction>;
}
