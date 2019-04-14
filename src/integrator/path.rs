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
    min_depth: i32,
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
        let mut ray = *ray;
        let mut specular_bounce = false;

        for bounces in 0..self.max_depth {
            let opt_isect = scene.intersect(&ray);

            match &opt_isect {
                Some(hit) => {
                    assert!(out.y() >= 0.0);
                    if bounces == 0 || specular_bounce {
                        out += beta
                            * hit
                                .light
                                .map(|l| l.radiance(&hit.int, -ray.d))
                                .unwrap_or_default();
                        assert!(out.y() >= 0.0);
                    }

                    let bsdf = hit.compute_bsdf(arena);
                    out += beta * self.uniform_sample_one(scene, sampler, &bsdf, &hit);
                    assert!(out.y() >= 0.0);

                    let wo = -ray.d;
                    let (f, wi, pdf, flags) = bsdf.sample(wo, BxDFType::ALL, sampler.get_2d());
                    if pdf == 0.0 || f.is_black() {
                        break;
                    }

                    specular_bounce = flags.contains(BxDFType::SPECULAR);
                    beta *= f * wi.dot_nrm(hit.shading.normal).abs() / pdf;
                    ray = hit.int.spawn_ray(wi);

                    if bounces > self.min_depth {
                        let q = (1.0 - beta.y()).max(0.05);
                        if sampler.get_1d() < q {
                            break;
                        }
                        beta = beta / (1.0 - q);
                    }
                }
                None => {
                    // TODO: Background radiance
                }
            }
        }

        out
    }
}
