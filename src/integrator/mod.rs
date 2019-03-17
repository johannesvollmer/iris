use crate::film::spectrum::Spectrum;
use crate::sampler::Sampler;
use crate::scene::Scene;
use crate::math::ray::Ray;

pub trait Integrator {
    fn radiance(&self, ray: &Ray, scene: &Scene, sampler: &Box<Sampler + Send + Sync>, depth: i32) -> Spectrum;
}