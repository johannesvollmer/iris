use crate::material::Material;
use crate::math::*;

pub mod primitive;
pub mod receiver;
pub mod sphere;

#[derive(Clone)]
pub struct LocalGeometry {
    pub point: LocalPoint3f,
    pub point_error: LocalVec3f,
    pub ns: LocalNormal3f,
    pub ng: LocalNormal3f,
    pub uv: Point2f,
    pub dpdu: LocalVec3f,
    pub dpdv: LocalVec3f,
    pub time: Float,
    pub t: Float,
}

impl LocalGeometry {
    pub fn into_global(self, m: &Transform, m_inv: &Transform) -> GlobalGeometry {
        let (p, err) =
            m.apply_point_with_error(self.point.as_global(), self.point_error.as_global());

        GlobalGeometry {
            point: p,
            point_error: err,
            ns: m_inv.apply_normal(self.ns.as_global()).normalized(),
            ng: m_inv.apply_normal(self.ng.as_global()).normalized(),
            uv: self.uv,
            dpdu: m.apply(self.dpdu.as_global()),
            dpdv: m.apply(self.dpdv.as_global()),
            time: self.time,
            t: self.t,
        }
    }
}

#[derive(Clone)]
pub struct GlobalGeometry {
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
    pub gg: GlobalGeometry,
    pub material: &'a dyn Material,
    pub geometry: &'a dyn Geometry,
}

impl<'a> HitInfo<'a> {
    pub fn spawn_ray(&self, dir: Vec3f) -> Ray {
        Ray::spawn(
            self.gg.point,
            dir,
            self.gg.point_error,
            self.gg.ng,
            self.gg.time,
        )
    }
}

pub trait LocalAABB {
    fn local_aabb(&self) -> Bounds3f;
}

pub trait Geometry: LocalAABB {
    fn local_intersect(
        &self,
        ray: &LocalRay,
        o_err: LocalVec3f,
        d_err: LocalVec3f,
    ) -> Option<LocalGeometry>;
}

pub trait AABB {
    fn aabb(&self) -> Bounds3f;
}

pub trait Hit {
    fn intersect(&self, ray: &Ray) -> Option<HitInfo>;
}
