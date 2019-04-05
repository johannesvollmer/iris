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
        self.transform.apply_bounds(self.geometry.aabb())
    }
}

impl Hit for Receiver {
    fn intersect(&self, ray: &Ray) -> Option<HitInfo> {
        let mut local_ray = self.inv_transform.apply_ray(ray);

        let mut hit = self.geometry.intersect_geometry(&mut local_ray)?;

        hit.point = self.transform.apply_point(hit.point);
        hit.ns = self.transform.apply(hit.ns.to_vec()).into();
        // hit.ns = self.inv_transform.apply_normal(hit.ns).normalized();
        hit.ng = self.transform.apply(hit.ng.to_vec()).into();
        // hit.ng = self.inv_transform.apply_normal(hit.ng).normalized();
        hit.dpdu = self.transform.apply(hit.dpdu);
        hit.dpdv = self.transform.apply(hit.dpdv);

        Some(HitInfo {
            lg: hit,
            material: &*self.material,
            geometry: &*self.geometry,
        })
    }
}
