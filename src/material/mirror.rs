use crate::bxdf::{bsdf::BSDF, fresnel, specular_reflection::SpecularReflection};
use crate::film::spectrum::Spectrum;
use crate::geometry::GlobalGeometry;
use crate::material::Material;
use crate::texture::Texture;
use bumpalo::Bump;
use std::sync::Arc;

#[derive(new)]
#[allow(dead_code)]
pub struct Mirror {
    kr: Arc<dyn Texture<Spectrum> + Send + Sync>,
}

impl Material for Mirror {
    fn bsdf<'a>(&self, hit: &GlobalGeometry, alloc: &'a Bump) -> BSDF<'a> {
        let mut bsdf = BSDF::new(hit);

        let r = self.kr.eval(hit);
        let fresnel = Arc::new(fresnel::NoOp::new());
        bsdf.push(alloc, SpecularReflection::new(r, fresnel));

        bsdf
    }
}
