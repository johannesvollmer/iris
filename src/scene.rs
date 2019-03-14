use crate::geometry::primitive::Primitive;
use crate::math::*;

pub struct Scene {
    geometry: Vec<Primitive>,
    bounds: Bounds3f,
}

impl Scene {
    pub fn new(geometry: Vec<Primitive>) -> Self {
        Self {
            geometry,
            bounds: Bounds3f::new(Point3f::new(0.0, 0.0, 0.0), Point3f::new(1.0, 1.0, 1.0)),
        }
    }

    pub fn world_bounds(&self) -> Bounds3f {
        self.bounds
    }
}
