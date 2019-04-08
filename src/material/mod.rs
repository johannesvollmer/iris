use crate::bxdf::bsdf::BSDF;
use crate::geometry::SurfaceInteraction;
use bumpalo::Bump;

pub mod matte;
pub mod mirror;
pub mod plastic;

pub trait Material {
    fn bsdf<'a>(&self, hit: &SurfaceInteraction, alloc: &'a Bump) -> BSDF<'a>;
}
