use super::Geometry;
use crate::bxdf::bsdf::BSDF;
use crate::material::Material;
use crate::math::*;

#[derive(Debug, Clone)]
pub struct Interaction {
    pub point: Point3f,
    pub point_error: Vec3f,
    pub normal: Normal3f,
    pub wo: Vec3f,
    pub time: Float,
}

#[derive(Debug, Clone)]
pub struct Shading {
    pub normal: Normal3f,
    pub dpdu: Vec3f,
    pub dpdv: Vec3f,
}

#[derive(Clone)]
pub struct SurfaceInteraction<'a> {
    pub int: Interaction,
    pub shading: Shading,
    pub uv: Point2f,
    pub dpdu: Vec3f,
    pub dpdv: Vec3f,
    pub ray_t: Float,
    pub bsdf: Option<&'a BSDF<'a>>,
    pub material: Option<&'a dyn Material>,
    pub geometry: Option<&'a (dyn Geometry + Send + Sync)>,
}

impl Interaction {
    pub fn spawn_ray(&self, dir: Vec3f) -> Ray {
        Ray::spawn(self.point, dir, self.point_error, self.normal, self.time)
    }
}
