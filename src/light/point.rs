#![allow(dead_code)]

use crate::film::spectrum::Spectrum;
use crate::geometry::{Hit, Interaction, SurfaceInteraction, AABB};
use crate::light::Light;
use crate::math::*;
use num::traits::FloatConst;

#[derive(Clone)]
pub struct Point {
    world_pos: Point3f,
    intensity: Spectrum,
    light_to_world: Transform,
}

impl Point {
    pub fn new(intensity: Spectrum, world_pos: Point3f) -> Self {
        Self {
            intensity,
            world_pos,
            light_to_world: Transform::translate(world_pos.to_vec()),
        }
    }
}

impl Light for Point {
    fn sample_incoming(
        &self,
        int: &Interaction,
        _samples: (Float, Float),
    ) -> (Spectrum, Vec3f, Float) {
        let dir = self.world_pos - int.point;
        (self.intensity / dir.length_squared(), dir, 1.0)
    }

    fn power(&self) -> Spectrum {
        self.intensity * 4.0 * Float::PI()
    }
}

impl AABB for Point {
    fn aabb(&self) -> Bounds3f {
        unreachable!()
    }
}

impl Hit for Point {
    fn intersect(&self, _ray: &Ray) -> Option<(SurfaceInteraction, Float)> {
        unreachable!()
    }
}
