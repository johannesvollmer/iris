use crate::bxdf::bsdf::BSDF;
use crate::geometry::GeometryHitInfo;
use bumpalo::Bump;

pub trait Material {
    fn bsdf<'a>(&self, hit: &GeometryHitInfo, alloc: &'a Bump) -> BSDF<'a>;
}

pub mod mirror;