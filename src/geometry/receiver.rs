use crate::math::*;
use super::{SurfaceInteraction, Geometry};
use std::sync::Arc;

#[derive(new, Clone)]
pub struct Receiver {
    geometry: Arc<Geometry>,
    transform: Transform,
}

impl Receiver {
    pub fn aabb(&self) -> Bounds3f {
        self.geometry.aabb()
    }

    pub fn intersect(&self, ray: &mut Ray) -> Option<SurfaceInteraction> {
        self.geometry.intersect(ray)
    }
}
