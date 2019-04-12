use super::{Geometry, Hit, SurfaceInteraction, AABB};
use crate::material::Material;
use crate::math::*;
use std::sync::Arc;

#[derive(Clone)]
pub struct Receiver {
    geometry: Arc<dyn Geometry + Send + Sync>,
    material: Arc<dyn Material + Send + Sync>,
    transform: TransformPair,
}

impl Receiver {
    pub fn new(
        geometry: Arc<dyn Geometry + Send + Sync>,
        material: Arc<dyn Material + Send + Sync>,
        obj_to_world: Transform,
    ) -> Self {
        Self {
            geometry,
            material,
            transform: TransformPair::from(obj_to_world),
        }
    }
}

impl AABB for Receiver {
    fn aabb(&self) -> Bounds3f {
        self.transform
            .to_global
            .apply_bounds(self.geometry.local_aabb())
    }
}

impl Hit for Receiver {
    fn intersect(&self, ray: &Ray) -> Option<(SurfaceInteraction, Float)> {
        let (local_ray, o_err, d_err) = self.transform.to_local.apply_ray_with_error(ray);

        let (lg, local_ray_t) = self.geometry.local_intersect(
            &local_ray.as_local(),
            o_err.as_local(),
            d_err.as_local(),
        )?;

        let si = lg.into_surface_interaction(
            &self.transform,
            ray,
            self.material.clone(),
            self.geometry.clone(),
            None,
        );

        Some((si, local_ray.as_local().global_t(local_ray_t, ray)))
    }
}
