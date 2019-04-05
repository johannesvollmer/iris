use crate::bxdf::{bsdf::BSDF, lambertian::Lambertian};
use crate::film::spectrum::Spectrum;
use crate::geometry::LocalGeometry;
use crate::material::Material;
use crate::math::*;
use crate::texture::Texture;
use bumpalo::Bump;
use std::sync::Arc;

#[derive(new)]
pub struct Matte {
    diffuse: Arc<dyn Texture<Spectrum> + Send + Sync>,
    roughness: Option<Arc<dyn Texture<Float> + Send + Sync>>,
}

impl Material for Matte {
    fn bsdf<'a>(&self, hit: &LocalGeometry, alloc: &'a Bump) -> BSDF<'a> {
        let diffuse = self.diffuse.eval(&hit);
        let roughness = self.roughness.as_ref().map(|r| r.eval(&hit)).unwrap_or(0.0);

        let mut bsdf = BSDF::new(hit);

        if roughness == 0.0 {
            bsdf.push(alloc, Lambertian::new(diffuse));
        } else {
            unimplemented!()
        }

        bsdf
    }
}
