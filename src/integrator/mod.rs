use crate::bxdf::{bsdf::BSDF, BxDFType};
use crate::film::spectrum::Spectrum;
use crate::geometry::HitInfo;
use crate::math::ray::Ray;
use crate::sampler::Sampler;
use crate::scene::Scene;
use bumpalo::Bump;

pub mod normals;
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

    fn specular_reflection(
        &self,
        ray: &Ray,
        _scene: &Scene,
        sampler: &mut (dyn Sampler + Send + Sync),
        _alloc: &Bump,
        bsdf: &BSDF,
        hit: &HitInfo,
        _depth: i32,
    ) -> Spectrum {
        let sample = sampler.get_2d();
        let ns = bsdf.ns.to_vec();
        let wo = -ray.d;
        let (f, wi, pdf, _types) = bsdf.sample(
            wo,
            BxDFType::REFLECTION | BxDFType::SPECULAR,
            (sample.x, sample.y),
        );

        let n_dot_wi = wi.dot(ns).abs();

        if pdf > 0.0 && !f.is_black() && n_dot_wi != 0.0 {
            let _reflected_ray = hit.spawn_ray(wi);
            //let li = self.radiance(&reflected_ray, scene, sampler, alloc, depth + 1);
            let li = Spectrum::from_rgb(1.0, 0.0, 0.0);
            f * li * n_dot_wi / pdf
        } else {
            Spectrum::all(0.0)
        }
    }
}
