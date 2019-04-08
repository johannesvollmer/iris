#![allow(dead_code)]

use crate::geometry::Geometry;
use crate::film::spectrum::Spectrum;
use std::sync::Arc;

#[derive(new, Clone)]
pub struct Diffuse {
    emission: Spectrum,
    area: Float,
    geometry: Arc<dyn Geometry + Send + Sync>,
    light_to_world: Transform,
    world_to_light: Transform,
}

impl AreaLight for Diffuse {
    fn radiance(point: Point3f, )
}
