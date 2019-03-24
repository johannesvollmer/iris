use bumpalo::Bump;
use super::Integrator;
use crate::film::spectrum::Spectrum;
use crate::math::*;
use crate::sampler::Sampler;
use crate::scene::Scene;

#[derive(new)]
pub struct Whitted {
    max_depth: i32,
}

impl Integrator for Whitted {
    fn radiance(
        &self,
        ray: &Ray,
        scene: &Scene,
        _sampler: &(dyn Sampler + Send + Sync),
        arena: &Bump,
        depth: i32,
    ) -> Spectrum {
        if depth > self.max_depth {
            return Spectrum::black();
        }

        let out = Spectrum::black();

        if let Some(hit) = scene.intersect(ray) {
            let _bsdf = hit.material.bsdf(&hit.geometry_hit_info, arena);

            // Evaluate contribution from lights

            // Evaluate specular contribution

        }

        out
    }
}
