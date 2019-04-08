use super::{Geometry, Hit, HitInfo, AABB};
use crate::material::Material;
use crate::math::*;
use std::sync::Arc;

#[derive(Clone)]
pub struct Receiver {
    geometry: Arc<dyn Geometry + Send + Sync>,
    material: Arc<dyn Material + Send + Sync>,
    obj_to_world: Transform,
    world_to_obj: Transform,
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
            obj_to_world,
            world_to_obj: obj_to_world.inverse(),
        }
    }
}

impl AABB for Receiver {
    fn aabb(&self) -> Bounds3f {
        self.obj_to_world.apply_bounds(self.geometry.local_aabb())
    }
}

impl Hit for Receiver {
    fn intersect(&self, ray: &Ray) -> Option<HitInfo> {
        let (local_ray, o_err, d_err) = self.world_to_obj.apply_ray_with_error(ray);

        let lg = self.geometry.local_intersect(
            &local_ray.as_local(),
            o_err.as_local(),
            d_err.as_local(),
        )?;

        Some(HitInfo {
            gg: lg.to_global(&self.obj_to_world, &self.world_to_obj),
            material: &*self.material,
            geometry: &*self.geometry,
        })
    }
}
