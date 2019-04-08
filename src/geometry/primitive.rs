use super::{receiver, SurfaceInteraction};
use crate::geometry::{Hit, AABB};
use crate::light::emitter;
use crate::math::*;
use bvh::aabb::Bounded;
use bvh::bounding_hierarchy::BHShape;

#[derive(Clone)]
pub enum Primitive {
    Receiver(receiver::Receiver),
    Emitter(emitter::Emitter),
}

impl AABB for Primitive {
    fn aabb(&self) -> Bounds3f {
        match self {
            Primitive::Emitter(e) => e.aabb(),
            Primitive::Receiver(r) => r.aabb(),
        }
    }
}

impl Hit for Primitive {
    fn intersect(&self, ray: &Ray) -> Option<SurfaceInteraction> {
        match self {
            Primitive::Emitter(e) => e.intersect(ray),
            Primitive::Receiver(r) => r.intersect(ray),
        }
    }
}

pub struct BVHPrimitive {
    primitive: Primitive,
    node_index: usize,
}

impl BVHPrimitive {
    pub fn new(primitive: Primitive) -> Self {
        Self {
            primitive,
            node_index: 0,
        }
    }

    pub fn intersect(&self, ray: &Ray) -> Option<SurfaceInteraction> {
        self.primitive.intersect(ray)
    }
}

impl BHShape for BVHPrimitive {
    fn set_bh_node_index(&mut self, index: usize) {
        self.node_index = index;
    }

    fn bh_node_index(&self) -> usize {
        self.node_index
    }
}

impl Bounded for BVHPrimitive {
    fn aabb(&self) -> bvh::aabb::AABB {
        self.primitive.aabb().to_aabb()
    }
}
