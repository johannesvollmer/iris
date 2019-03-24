use crate::material::Material;
use super::{Geometry, HitInfo, AABB, Hit};
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
    pub fn new(geometry: Arc<dyn Geometry + Send + Sync>, material: Arc<dyn Material + Send + Sync>, transform: Transform) -> Self {
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
        self.geometry.aabb()
    }
}

impl Hit for Receiver {
    fn intersect(&self, ray: &Ray) -> Option<HitInfo> {
        let mut local_ray = self.inv_transform.apply_ray(ray);

        let hit = self.geometry.intersect_geometry(&mut local_ray)?;

        Some(HitInfo {
            geometry_hit_info: hit,
            material: &*self.material,
            geometry: &*self.geometry,
        })
    }
}