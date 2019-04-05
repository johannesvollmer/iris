use super::{Geometry, Hit, HitInfo, AABB};
use crate::material::Material;
use crate::math::*;
use std::sync::Arc;

#[derive(Clone)]
pub struct Receiver {
    geometry: Arc<dyn Geometry + Send + Sync>,
    material: Arc<dyn Material + Send + Sync>,
    transform: Transform,
    inv_transform: Transform,
}

impl Receiver {
    pub fn new(
        geometry: Arc<dyn Geometry + Send + Sync>,
        material: Arc<dyn Material + Send + Sync>,
        transform: Transform,
    ) -> Self {
        Self {
            geometry,
            material,
            transform,
            inv_transform: transform.inverse(),
        }
    }
}

impl AABB for Receiver {
    fn aabb(&self) -> Bounds3f {
        self.transform.apply_bounds(self.geometry.local_aabb())
    }
}

impl Hit for Receiver {
    fn intersect(&self, ray: &Ray) -> Option<HitInfo> {
        let mut local_ray = self.inv_transform.apply_ray(ray).as_local();

        let lg = self.geometry.local_intersect(&mut local_ray)?;

        Some(HitInfo {
            gg: lg.to_global(&self.transform, &self.inv_transform),
            material: &*self.material,
            geometry: &*self.geometry,
        })
    }
}
