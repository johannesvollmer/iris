use crate::material::Material;
use crate::math::*;
use std::sync::Arc;

pub mod interaction;
pub mod primitive;
pub mod receiver;

pub mod disk;
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
        m: &TransformPair,
        ray: &Ray,
        material: Arc<dyn Material + Send + Sync>,
        geometry: Arc<dyn Geometry + Send + Sync>,
    ) -> SurfaceInteraction<'a> {
        let (p, err) = m
            .to_global
            .apply_point_with_error(self.point.as_global(), self.point_error.as_global());

        SurfaceInteraction {
            int: Interaction {
                point: p,
                point_error: err,
                // TODO: Face forward correction
                normal: m.to_local.apply_normal(self.ng.as_global()).normalized(),
                wo: -ray.d,
                time: self.time,
            },
            shading: Shading {
                // TODO: Face forward correction
                normal: m.to_local.apply_normal(self.ns.as_global()).normalized(),
                dpdu: m.to_global.apply(self.dpdu.as_global()),
                dpdv: m.to_global.apply(self.dpdu.as_global()),
            },
            uv: self.uv,
            dpdu: m.to_global.apply(self.dpdu.as_global()),
            dpdv: m.to_global.apply(self.dpdv.as_global()),
            bsdf: None,
            material: Some(material),
            geometry: Some(geometry),
        }
    }
}

pub trait LocalAABB {
    fn local_aabb(&self) -> Bounds3f;
}

pub trait Geometry: LocalAABB + IntoGeometry {
    fn local_intersect(
        &self,
        ray: &LocalRay,
        o_err: LocalVec3f,
        d_err: LocalVec3f,
    ) -> Option<(LocalGeometry, Float)>;

    fn area(&self) -> Float;
}

pub trait Sampleable: Geometry {
    fn sample_shape(
        &self,
        int: &Interaction,
        transform: &TransformPair,
        samples: (Float, Float),
    ) -> Point3f;

    fn pdf(&self, _int: &Interaction, _transform: &TransformPair, _dir: Vec3f) -> Float {
        1.0 / self.area()
    }
}

pub trait AABB {
    fn aabb(&self) -> Bounds3f;
}

pub trait Hit {
    fn intersect(&self, ray: &Ray) -> Option<(SurfaceInteraction, Float)>;
}

// Allows casting Arc<dyn Sampleable> -> Arc<dyn Geometry>, for example.
pub trait IntoGeometry {
    fn into_geometry(self: Arc<Self>) -> Arc<dyn Geometry + Send + Sync>;
}

impl<T: Geometry + Send + Sync + 'static> IntoGeometry for T {
    fn into_geometry(self: Arc<Self>) -> Arc<dyn Geometry + Send + Sync> {
        self
    }
}
