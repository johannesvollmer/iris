use crate::material::Material;
use crate::math::*;

pub mod primitive;
pub mod receiver;
pub mod sphere;

pub struct GeometryHitInfo {
    pub point: Point3f,
    pub ns: Normal3f,
    pub ng: Normal3f,
    pub uv: Point2f,
    pub dpdu: Vec3f,
    pub dpdv: Vec3f,
    pub time: Float,
    pub t: Float,
}

pub struct HitInfo<'a> {
    pub geometry_hit_info: GeometryHitInfo,
    pub material: &'a dyn Material,
    pub geometry: &'a dyn Geometry,
}

pub trait AABB {
    fn aabb(&self) -> Bounds3f;
}

pub trait Geometry: AABB {
    fn intersect_geometry(&self, ray: &Ray) -> Option<GeometryHitInfo>;
}

pub trait Hit {
    fn intersect(&self, ray: &Ray) -> Option<HitInfo>;
}
