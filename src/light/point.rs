#![allow(dead_code)]

use crate::film::spectrum::Spectrum;
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
    fn sample(&self, to: Point3f, _samples: (Float, Float)) -> (Spectrum, LocalPoint3f, Float) {
        let dir = self.world_pos - to;
        (
            self.intensity / dir.length_squared(),
            LocalPoint3f::default(),
            1.0,
        )
    }

    fn power(&self) -> Spectrum {
        self.intensity * 4.0 * Float::PI()
    }

    fn light_to_world(&self) -> &Transform {
        &self.light_to_world
    }
}
