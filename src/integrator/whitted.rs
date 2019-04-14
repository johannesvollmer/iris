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

            if let Some(e) = hit.light {
                out += e.radiance(&hit.int, wo);
            }

            // Evaluate contribution from lights
            for light in &scene.lights {
                let (li, wi, pdf, vis) = light.sample_incoming(&hit.int, sampler.get_2d());
                let f = bsdf.eval(wo, wi, BxDFType::ALL);
                // if !li.is_black() && !f.is_black() && vis.visible(scene) {
                //     out += f * li * wi.dot_nrm(bsdf.ns).abs() / pdf;
                //     let dot = wi.dot_nrm(bsdf.ns);
                //     dbg!(bsdf.ng);
                //     dbg!(wi);
                //     assert!(dot >= 0.0);
                //     assert!(dot <= 1.0);
                //     out = Spectrum::all(1.0) * dot.powi(10);
                // }
                if !li.is_black() && pdf != 0.0 && vis.visible(scene) {
                    let dot = wi.dot_nrm(bsdf.ns).max(0.0);
                    let diffuse = li * f * dot / pdf;
                    // dbg!(wi);
                    // dbg!(bsdf.ns);
                    // dbg!(li);
                    // dbg!(dot);
                    // dbg!(pdf);
                    // dbg!(diffuse);
                    // dbg!(dot);
                    out += diffuse;
                }
            }

            // Evaluate specular contribution
            // out += self.specular_reflection(ray, scene, sampler, arena, &bsdf, &hit, depth);
        }

        out
    }
}
