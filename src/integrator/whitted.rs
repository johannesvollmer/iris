use super::Integrator;
use crate::film::spectrum::Spectrum;
use crate::math::*;
use crate::sampler::Sampler;
use crate::scene::Scene;

#[derive(new)]
pub struct Whitted {
    max_depth: i32,
}

impl Integrator for Whitted {
    fn radiance(
        &self,
        ray: &mut Ray,
        scene: &Scene,
        sampler: &Box<Sampler + Send + Sync>,
        depth: i32,
    ) -> Spectrum {
        if let Some(hit) = scene.intersect(ray) {
            let c = Spectrum::from_rgb(1.0, 1.0, 1.0);
            let scale = hit.normal.dot(&ray.d).abs().powf(0.7);
            c * scale
        } else {
            Spectrum::from_rgb(0.0, 0.0, 0.0)
        }
    }
}
