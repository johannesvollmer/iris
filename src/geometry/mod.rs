use crate::light::Light;
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
        light: Option<&'a (dyn Light + Send + Sync)>,
    ) -> SurfaceInteraction<'a> {
        let (p, err) = m
            .to_global
            .apply_point_with_error(self.point.as_global(), self.point_error.as_global());

        let ng = m.to_local.apply_normal(self.ng.as_global()).normalized();
        let ns = m
            .to_local
            .apply_normal(self.ns.as_global())
            .normalized()
            .face_forward(ng);

        SurfaceInteraction {
            int: Interaction {
                point: p,
                point_error: err,
                normal: ng,
                wo: -ray.d,
                time: self.time,
            },
            shading: Shading {
                normal: ns,
                dpdu: m.to_global.apply(self.dpdu.as_global()),
                dpdv: m.to_global.apply(self.dpdu.as_global()),
            },
            uv: self.uv,
            dpdu: m.to_global.apply(self.dpdu.as_global()),
            dpdv: m.to_global.apply(self.dpdv.as_global()),
            bsdf: None,
            material: Some(material),
            geometry: Some(geometry),
            light,
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
    ) -> Interaction;

    fn pdf(&self, int: &Interaction, transform: &TransformPair, dir: Vec3f) -> Float {
        let ray = int.spawn_ray(dir);
        let (local_ray, o_err, d_err) = transform.to_local.apply_ray_with_error(&ray);
        match self.local_intersect(&local_ray.as_local(), o_err.as_local(), d_err.as_local()) {
            Some((lg, _)) => {
                let hit_point = transform.to_global.apply_point(lg.point.as_global());
                let hit_normal = transform.to_local.apply_normal(lg.ng.as_global());
                int.point.distance_squared(hit_point) / ((-dir).dot_nrm(hit_normal) * self.area())
            }
            None => 0.0,
        }
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
