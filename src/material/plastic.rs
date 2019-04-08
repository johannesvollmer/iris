use crate::bxdf::microfacet::trowbridge_reitz::TrowbridgeReitz;
use crate::bxdf::{
    bsdf::BSDF, fresnel, lambertian::Lambertian, microfacet,
    microfacet_reflection::MicrofacetReflection,
};
use crate::film::spectrum::Spectrum;
use crate::geometry::GlobalGeometry;
use crate::material::Material;
use crate::math::*;
use crate::texture::Texture;
use bumpalo::Bump;
use std::sync::Arc;

#[derive(new)]
#[allow(dead_code)]
pub struct Plastic {
    kd: Arc<dyn Texture<Spectrum> + Send + Sync>,
    ks: Arc<dyn Texture<Spectrum> + Send + Sync>,
    roughness: Arc<dyn Texture<Float> + Send + Sync>,
}

impl Material for Plastic {
    fn bsdf<'a>(&self, hit: &GlobalGeometry, alloc: &'a Bump) -> BSDF<'a> {
        let mut bsdf = BSDF::new(hit);

        let kd = self.kd.eval(hit);
        bsdf.push(alloc, Lambertian::new(kd));

        let ks = self.ks.eval(hit);
        let roughness = microfacet::roughness_to_alpha(self.roughness.eval(hit));
        let distrib = Arc::new(TrowbridgeReitz::new(roughness, roughness));
        let fresnel = Arc::new(fresnel::Dielectric::new(1.5, 1.0));
        bsdf.push(alloc, MicrofacetReflection::new(ks, distrib, fresnel));

        bsdf
    }
}
