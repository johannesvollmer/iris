use crate::bxdf::bsdf::BSDF;
use crate::geometry::GlobalGeometry;
use bumpalo::Bump;

pub trait Material {
    fn bsdf<'a>(&self, hit: &GlobalGeometry, alloc: &'a Bump) -> BSDF<'a>;
}

pub mod matte;
pub mod mirror;
