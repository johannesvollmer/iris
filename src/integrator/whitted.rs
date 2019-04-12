use super::Integrator;
use crate::bxdf::BxDFType;
use crate::film::spectrum::Spectrum;
use crate::math::*;
use crate::sampler::Sampler;
use crate::scene::Scene;
use bumpalo::Bump;

#[derive(new)]
#[allow(dead_code)]
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
            let bsdf = hit.compute_bsdf(arena);

            let wo = -ray.d;

            // Evaluate contribution from lights
            for light in &scene.lights {
                let (li, wi, pdf, vis) = light.sample_incoming(&hit.int, sampler.get_2d());
                let f = bsdf.eval(wo, wi, BxDFType::ALL);
                if !li.is_black() && !f.is_black() && vis.visible(scene) {
                    out += f * li * wi.dot_nrm(bsdf.ns).abs() / pdf;
                }
            }

            // Evaluate specular contribution
            out += self.specular_reflection(ray, scene, sampler, arena, &bsdf, &hit, depth);
        }

        out
    }
}
