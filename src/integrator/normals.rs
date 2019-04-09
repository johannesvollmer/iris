use crate::film::spectrum::Spectrum;
use crate::integrator::Integrator;
use crate::math::*;
use crate::sampler::Sampler;
use crate::scene::Scene;
use bumpalo::Bump;

#[derive(new)]
#[allow(dead_code)]
pub struct Normals;

impl Integrator for Normals {
    fn radiance(
        &self,
        ray: &Ray,
        scene: &Scene,
        _sampler: &mut (dyn Sampler + Send + Sync),
        arena: &Bump,
        _depth: i32,
    ) -> Spectrum {
        if let Some(hit) = scene.intersect(ray) {
            let bsdf = hit.compute_bsdf(arena);
            let ng = bsdf.ng.normalized();
            Spectrum::from_rgb(ng.x, ng.y, ng.z) / 2.0 + 0.5
        } else {
            Spectrum::all(0.0)
        }
    }
}
