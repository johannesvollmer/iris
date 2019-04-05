use crate::bxdf::bsdf::BSDF;
use crate::geometry::LocalGeometry;
use bumpalo::Bump;

pub trait Material {
    fn bsdf<'a>(&self, hit: &LocalGeometry, alloc: &'a Bump) -> BSDF<'a>;
}

pub mod matte;
pub mod mirror;
