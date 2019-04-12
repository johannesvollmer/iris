use crate::bxdf::BxDFType;
use crate::film::spectrum::Spectrum;
use crate::integrator::Integrator;
use crate::math::*;
use crate::sampler::Sampler;
use crate::scene::Scene;
use bumpalo::Bump;

#[derive(new)]
#[allow(dead_code)]
pub struct Path {
    max_depth: i32,
}

impl Integrator for Path {
    fn radiance(
        &self,
        ray: &Ray,
        scene: &Scene,
        sampler: &mut (dyn Sampler + Send + Sync),
        arena: &Bump,
        _depth: i32,
    ) -> Spectrum {
        let mut out = Spectrum::default();
        let mut beta = Spectrum::all(1.0);
        let mut ray = ray.clone();
        let mut specular_bounce = false;

        for bounces in 0..self.max_depth {
            let opt_isect = scene.intersect(&ray);

            if bounces == 0 || specular_bounce {
                out += match &opt_isect {
                    Some(isect) => {
                        beta * isect
                            .light
                            .map(|l| l.radiance(&isect.int, -ray.d))
                            .unwrap_or_default()
                    }
                    None => Spectrum::default(), // TODO: Background radiance
                };
            }
            if opt_isect.is_none() || bounces >= self.max_depth {
                break;
            }

            let hit = opt_isect.unwrap();
            let bsdf = hit.compute_bsdf(arena);
            // TODO: Account for things without BSDF
            out += beta * self.uniform_sample_one(scene, sampler, &bsdf, &hit);

            let wo = -ray.d;
            let (f, wi, pdf, flags) = bsdf.sample(wo, BxDFType::ALL, sampler.get_2d());
            if pdf == 0.0 || f.is_black() {
                break;
            }

            beta *= f * wi.dot_nrm(hit.shading.normal).abs() / pdf;
            specular_bounce = flags.contains(BxDFType::SPECULAR);
            ray = hit.int.spawn_ray(wi);

            if bounces > 3 {
                let q = (1.0 - beta.y()).max(0.05);
                beta = beta / (1.0 - q);
                if sampler.get_1d() < q {
                    break;
                }
            }
        }

        out
    }
}
