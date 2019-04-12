use crate::bxdf::{bsdf::BSDF, BxDFType};
use crate::film::spectrum::Spectrum;
use crate::geometry::SurfaceInteraction;
use crate::light::emitter::Emitter;
use crate::light::Light;
use crate::math::ray::Ray;
use crate::math::*;
use crate::sampler::Sampler;
use crate::scene::Scene;
use bumpalo::Bump;

pub mod normals;
pub mod path;
pub mod whitted;

pub trait Integrator {
    fn radiance(
        &self,
        ray: &Ray,
        scene: &Scene,
        sampler: &mut (dyn Sampler + Send + Sync),
        alloc: &Bump,
        depth: i32,
    ) -> Spectrum;

    #[allow(clippy::too_many_arguments)]
    fn specular_reflection(
        &self,
        ray: &Ray,
        scene: &Scene,
        sampler: &mut (dyn Sampler + Send + Sync),
        alloc: &Bump,
        bsdf: &BSDF,
        hit: &SurfaceInteraction,
        depth: i32,
    ) -> Spectrum {
        let ng = bsdf.ng.to_vec();
        let wo = -ray.d;
        let (f, wi, pdf, _types) = bsdf.sample(
            wo,
            BxDFType::REFLECTION | BxDFType::SPECULAR,
            sampler.get_2d(),
        );

        let n_dot_wi = wi.dot(ng).abs();

        if pdf > 0.0 && !f.is_black() && n_dot_wi != 0.0 {
            let reflected_ray = hit.int.spawn_ray(wi);
            let li = self.radiance(&reflected_ray, scene, sampler, alloc, depth + 1);
            f * li * n_dot_wi / pdf
        } else {
            Spectrum::all(0.0)
        }
    }

    fn uniform_sample_all(
        &self,
        scene: &Scene,
        sampler: &mut (dyn Sampler + Send + Sync),
        bsdf: &BSDF,
        hit: &SurfaceInteraction,
        n_samples: i32,
    ) -> Spectrum {
        let mut out = Spectrum::default();

        for light in &scene.lights {
            out += (0..n_samples)
                .map(|_| self.estimate_direct(light, scene, sampler, bsdf, hit))
                .sum::<Spectrum>()
                / (n_samples as Float);
        }

        out
    }

    fn uniform_sample_one(
        &self,
        scene: &Scene,
        sampler: &mut (dyn Sampler + Send + Sync),
        bsdf: &BSDF,
        hit: &SurfaceInteraction,
    ) -> Spectrum {
        let n_lights = scene.lights.len();
        if n_lights == 0 {
            return Spectrum::default();
        }

        let chosen = ((sampler.get_1d() * n_lights as Float) as usize).min(n_lights - 1);
        let light = &scene.lights[chosen];

        self.estimate_direct(light, scene, sampler, bsdf, hit) * (n_lights as Float)
    }

    fn estimate_direct(
        &self,
        light: &Emitter,
        scene: &Scene,
        sampler: &mut (dyn Sampler + Send + Sync),
        bsdf: &BSDF,
        hit: &SurfaceInteraction,
    ) -> Spectrum {
        let mut out = Spectrum::default();
        let flags = BxDFType::ALL & !BxDFType::SPECULAR;

        let (li, wi, mut light_pdf, vis) = light.sample_incoming(&hit.int, sampler.get_2d());

        if light_pdf > 0.0 && !li.is_black() {
            let f = bsdf.eval(hit.int.wo, wi, flags) * wi.dot_nrm(hit.shading.normal).abs();
            let scattering_pdf = bsdf.pdf(hit.int.wo, wi, flags);
            if !f.is_black() && vis.visible(scene) {
                let weight = if light.is_delta() {
                    1.0
                } else {
                    power_heuristic(1, light_pdf, 1, scattering_pdf)
                };
                out += f * li * weight / light_pdf;
            }
        }

        if !light.is_delta() {
            let (mut f, wi, scattering_pdf, sampled_flags) =
                bsdf.sample(hit.int.wo, flags, sampler.get_2d()); // TODO: Flags?
            f *= wi.dot_nrm(hit.shading.normal);
            let sampled_specular = sampled_flags.contains(BxDFType::SPECULAR);

            if f.is_black() || scattering_pdf <= 0.0 {
                return out;
            }

            let mut weight = 1.0;
            if sampled_specular {
                light_pdf = light.pdf_incoming(&hit.int, wi);
                if light_pdf == 0.0 {
                    return out;
                }
                weight = power_heuristic(1, scattering_pdf, 1, light_pdf);
            }

            let ray = hit.int.spawn_ray(wi);
            let li = match scene.intersect(&ray) {
                Some(isect) => {
                    let mut li = Spectrum::default();
                    // TODO: This is a hack, please fix
                    if let Some(e) = isect.light {
                        if e as *const Light == &*light.light as *const Light {
                            li = e.radiance(&isect.int, -wi);
                        }
                    }
                    li
                }
                _ => Spectrum::default(), // Nothing hit, return background radiance
            };

            if !li.is_black() {
                out += f * li * weight / scattering_pdf;
            }
        }

        out
    }
}
