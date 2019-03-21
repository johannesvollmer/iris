use super::{Geometry, HitInfo};
use crate::math::*;
use std::sync::Arc;

#[derive(Clone)]
pub struct Receiver {
    geometry: Arc<dyn Geometry + Send + Sync>,
    transform: Transform,
    inv_transform: Transform,
}

impl Receiver {
    pub fn new(geometry: Arc<dyn Geometry + Send + Sync>, transform: Transform) -> Self {
        Self {
            geometry,
            transform,
            inv_transform: transform.inverse(),
        }
    }
}

impl Geometry for Receiver {
    fn aabb(&self) -> Bounds3f {
        self.geometry.aabb()
    }

    fn intersect(&self, ray: &mut Ray) -> Option<HitInfo> {
        let mut local_ray = self.inv_transform.apply_ray(ray);

        let hit = self.geometry.intersect(&mut local_ray)?;

        ray.t_max = local_ray.t_max;
        Some(hit)
    }
}
