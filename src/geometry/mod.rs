use crate::material::Material;
use crate::math::*;

pub mod primitive;
pub mod receiver;
pub mod sphere;

#[derive(Clone)]
pub struct GeometryHitInfo {
    pub point: Point3f,
    pub point_error: Vec3f,
    pub ns: Normal3f,
    pub ng: Normal3f,
    pub uv: Point2f,
    pub dpdu: Vec3f,
    pub dpdv: Vec3f,
    pub time: Float,
    pub t: Float,
}

#[derive(Clone)]
pub struct HitInfo<'a> {
    pub geometry_hit_info: GeometryHitInfo,
    pub material: &'a dyn Material,
    pub geometry: &'a dyn Geometry,
}

impl<'a> HitInfo<'a> {
    pub fn spawn_ray(&self, dir: Vec3f) -> Ray {
        let gh = &self.geometry_hit_info;
        Ray::spawn(gh.point, dir, gh.point_error, gh.ng, gh.time)
    }

    /*pub fn spawn_ray_to(&self, point: Point3f) -> Ray {
        let gh = &self.geometry_hit_info;
        Ray::spawn_to(gh.point, point, gh.point_error, gh.ng, gh.time)
    }*/
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
