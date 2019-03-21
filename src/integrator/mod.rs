use crate::film::spectrum::Spectrum;
use crate::math::ray::Ray;
use crate::sampler::Sampler;
use crate::scene::Scene;

pub mod whitted;

pub trait Integrator {
    fn radiance(
        &self,
        ray: &mut Ray,
        scene: &Scene,
        sampler: &Box<Sampler + Send + Sync>,
        depth: i32,
    ) -> Spectrum;
}
