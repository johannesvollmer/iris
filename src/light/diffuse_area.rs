#![allow(dead_code)]

use crate::film::spectrum::Spectrum;
use crate::geometry::{Hit, Interaction, Sampleable, SurfaceInteraction, AABB};
use crate::light::Light;
use crate::material::Material;
use crate::math::*;
use std::sync::Arc;

#[derive(Clone)]
pub struct DiffuseArea {
    emission: Spectrum,
    geometry: Arc<dyn Sampleable + Send + Sync>,
    material: Arc<dyn Material + Send + Sync>,
    transform: TransformPair,
}

impl DiffuseArea {
    pub fn new(
        emission: Spectrum,
        transform: Transform,
        geometry: Arc<dyn Sampleable + Send + Sync>,
        material: Arc<dyn Material + Send + Sync>,
    ) -> Self {
        Self {
            emission,
            geometry,
            material,
            transform: TransformPair::from(transform),
        }
    }
}

impl Light for DiffuseArea {
    fn radiance(&self, light_int: &Interaction, w: Vec3f) -> Spectrum {
        if w.dot_nrm(light_int.normal) > 0.0 {
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
        let light_int = self.geometry.sample_shape(int, &self.transform, samples);

        let dir = light_int.point - int.point;
        let pdf = self.geometry.pdf(int, &self.transform, dir);

        (self.radiance(&light_int, -dir), dir, pdf)
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
        self.transform
            .to_global
            .apply_bounds(self.geometry.local_aabb())
    }
}

impl Hit for DiffuseArea {
    fn intersect(&self, ray: &Ray) -> Option<(SurfaceInteraction, Float)> {
        let (local_ray, o_err, d_err) = self.transform.to_local.apply_ray_with_error(ray);

        let (lg, local_ray_t) = self.geometry.local_intersect(
            &local_ray.as_local(),
            o_err.as_local(),
            d_err.as_local(),
        )?;

        let si = lg.into_surface_interaction(
            &self.transform,
            ray,
            self.material.clone(),
            self.geometry.clone().into_geometry(),
            Some(self),
        );

        Some((si, local_ray.as_local().global_t(local_ray_t, ray)))
    }
}
