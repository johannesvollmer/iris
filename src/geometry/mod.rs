use crate::math::*;

pub mod interaction;
pub mod primitive;
pub mod receiver;

pub mod sphere;

pub use interaction::{Interaction, Shading, SurfaceInteraction};

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
}

impl LocalGeometry {
    pub fn into_surface_interaction<'a>(
        self,
        m: &Transform,
        m_inv: &Transform,
        ray: &Ray,
    ) -> SurfaceInteraction<'a> {
        let (p, err) =
            m.apply_point_with_error(self.point.as_global(), self.point_error.as_global());

        SurfaceInteraction {
            int: Interaction {
                point: p,
                point_error: err,
                // TODO: Face forward correction
                normal: m_inv.apply_normal(self.ng.as_global()).normalized(),
                wo: -ray.d,
                time: self.time,
            },
            shading: Shading {
                // TODO: Face forward correction
                normal: m_inv.apply_normal(self.ns.as_global()).normalized(),
                dpdu: m.apply(self.dpdu.as_global()),
                dpdv: m.apply(self.dpdu.as_global()),
            },
            uv: self.uv,
            dpdu: m.apply(self.dpdu.as_global()),
            dpdv: m.apply(self.dpdv.as_global()),
            bsdf: None,
            material: None,
            geometry: None,
        }
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
    ) -> Option<(LocalGeometry, Float)>;
}

pub trait Sampleable: Geometry {
    fn sample_shape(
        &self,
        int: &Interaction,
        transform: &TransformPair,
        samples: (Float, Float),
    ) -> Point3f;

    fn pdf(&self, int: &Interaction, transform: &TransformPair, dir: Vec3f) -> Float;
}

pub trait AABB {
    fn aabb(&self) -> Bounds3f;
}

pub trait Hit {
    fn intersect(&self, ray: &Ray) -> Option<(SurfaceInteraction, Float)>;
}
