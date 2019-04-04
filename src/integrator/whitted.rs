use super::Integrator;
use crate::bxdf::BxDFType;
use crate::film::spectrum::Spectrum;
use crate::math::*;
use crate::sampler::Sampler;
use crate::scene::Scene;
use bumpalo::Bump;

#[derive(new)]
pub struct Whitted {
    max_depth: i32,
}

impl Integrator for Whitted {
    fn radiance(
        &self,
        ray: &Ray,
        scene: &Scene,
        sampler: &mut (dyn Sampler + Send + Sync),
        arena: &Bump,
        depth: i32,
    ) -> Spectrum {
        if depth > self.max_depth {
            return Spectrum::black();
        }

        let mut out = Spectrum::black();

        if let Some(hit) = scene.intersect(ray) {
            let bsdf = hit.material.bsdf(&hit.geometry_hit_info, arena);

            // Evaluate contribution from lights

            // Evaluate specular contribution
            let specular = {
                let sample = sampler.get_2d();
                let ns = hit.geometry_hit_info.ns.to_vec();
                let wo = -ray.d;
                let (spectrum, wi, pdf, _types) = bsdf.sample(
                    &wo,
                    BxDFType::REFLECTION | BxDFType::SPECULAR,
                    (sample.x, sample.y),
                );
                spectrum * Spectrum::from_rgb(1.0, 0.0, 0.0) * (wi.dot(&ns).abs() / pdf)
            };

            out += specular;
        }

        out.clamp(0.0, 1.0)
    }
}
