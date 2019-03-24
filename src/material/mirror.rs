use bumpalo::Bump;
use crate::bxdf::bsdf::BSDF;
use crate::material::Material;
use crate::geometry::GeometryHitInfo;

#[derive(new)]
pub struct Mirror {}

impl Material for Mirror {
    fn bsdf<'a>(&self, hit: &GeometryHitInfo, _alloc: &'a Bump) -> BSDF<'a> {
        let mut bsdf = BSDF::new(hit);
        bsdf
    }
}