#![allow(dead_code)]

use crate::film::spectrum::Spectrum;
use crate::geometry::{Hit, Interaction, Sampleable, SurfaceInteraction, AABB};
use crate::light::Light;
use crate::math::*;
use std::sync::Arc;

#[derive(Clone)]
pub struct DiffuseArea {
    emission: Spectrum,
    geometry: Arc<dyn Sampleable + Send + Sync>,
    transform: TransformPair,
}

impl DiffuseArea {
    pub fn new(
        emission: Spectrum,
        transform: Transform,
        geometry: Arc<dyn Sampleable + Send + Sync>,
    ) -> Self {
        Self {
            emission,
            geometry,
            transform: TransformPair::from(transform),
        }
    }
}

impl Light for DiffuseArea {
    fn radiance(&self, int: &Interaction, w: Vec3f) -> Spectrum {
        if w.dot_nrm(int.normal) > 0.0 {
            self.emission
        } else {
            Spectrum::all(0.0)
        }
    }

    fn sample_incoming(
        &self,
        int: &Interaction,
        samples: (Float, Float),
    ) -> (Spectrum, Vec3f, Float) {
        let world_sample_point = self.geometry.sample_shape(int, &self.transform, samples);

        let dir = world_sample_point - int.point;
        let pdf = self.geometry.pdf(int, &self.transform, dir);

        (self.radiance(int, dir), dir, pdf)
    }

    fn power(&self) -> Spectrum {
        unimplemented!()
    }

    fn pdf_incoming(&self, int: &Interaction, wi: Vec3f) -> Float {
        self.geometry.pdf(int, &self.transform, wi)
    }
}

impl AABB for DiffuseArea {
    fn aabb(&self) -> Bounds3f {
        dbg!(self.transform
            .to_global
            .apply_bounds(self.geometry.local_aabb()))
    }
}

impl Hit for DiffuseArea {
    fn intersect(&self, _ray: &Ray) -> Option<(SurfaceInteraction, Float)> {
        None
    }
}
