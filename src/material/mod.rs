use crate::bxdf::bsdf::BSDF;
use crate::geometry::GlobalGeometry;
use bumpalo::Bump;

pub mod matte;
pub mod mirror;
pub mod plastic;

pub trait Material {
    fn bsdf<'a>(&self, hit: &GlobalGeometry, alloc: &'a Bump) -> BSDF<'a>;
}
