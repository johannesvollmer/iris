use bumpalo::Bump;
use crate::bxdf::bsdf::BSDF;
use crate::film::spectrum::Spectrum;
use crate::math::ray::Ray;
use crate::sampler::Sampler;
use crate::scene::Scene;

pub mod whitted;

pub trait Integrator {
    fn radiance(
        &self,
        ray: &Ray,
        scene: &Scene,
        sampler: &(dyn Sampler + Send + Sync),
        alloc: &Bump,
        depth: i32,
    ) -> Spectrum;

    fn specular_reflection(
        &self,
        _ray: &Ray,
        _scene: &Scene,
        _sampler: &(dyn Sampler + Send + Sync),
        _alloc: &Bump,
        _bsdf: &BSDF,
        _depth: i32,
    ) -> Spectrum {
        Spectrum::black()
    }
}
