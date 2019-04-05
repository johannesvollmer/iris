use super::Integrator;
use crate::bxdf::BxDFType;
use crate::film::spectrum::Spectrum;
use crate::light::Light;
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
            let bsdf = hit.material.bsdf(&hit.gg, arena);

            let wo = -ray.d;
            let sample = {
                let v = sampler.get_2d();
                (v.x, v.y)
            };

            // Evaluate contribution from lights
            for light in &scene.lights {
                let (li, wi, pdf, vis) = light.sample(&hit, sample);
                let f = bsdf.eval(wo, wi, BxDFType::ALL);
                if !li.is_black() && !f.is_black() && vis.visible(scene) {
                    out += f * li * wi.dot(bsdf.ns.to_vec()) / pdf;
                }
            }

            // Evaluate specular contribution
            out += self.specular_reflection(ray, scene, sampler, arena, &bsdf, &hit, depth);
        }

        out.clamp(0.0, 1.0)
    }
}
