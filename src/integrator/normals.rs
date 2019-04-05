use crate::film::spectrum::Spectrum;
use crate::integrator::Integrator;
use crate::math::*;
use crate::sampler::Sampler;
use crate::scene::Scene;
use bumpalo::Bump;

#[derive(new)]
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
            let bsdf = hit.material.bsdf(&hit.lg, arena);
            let ns = bsdf.ns.to_vec().normalized();
            Spectrum::from_rgb(ns.x, ns.y, ns.z) / 2.0 + 0.5
        } else {
            Spectrum::all(0.0)
        }
    }
}
