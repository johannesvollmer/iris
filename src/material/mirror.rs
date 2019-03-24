use crate::bxdf::bsdf::BSDF;
use crate::geometry::GeometryHitInfo;
use crate::material::Material;
use bumpalo::Bump;

#[derive(new)]
pub struct Mirror {}

impl Material for Mirror {
    fn bsdf<'a>(&self, hit: &GeometryHitInfo, _alloc: &'a Bump) -> BSDF<'a> {
        let mut bsdf = BSDF::new(hit);
        bsdf
    }
}
