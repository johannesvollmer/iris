use bumpalo::Bump;
use crate::geometry::GeometryHitInfo;
use super::BxDF;
use crate::math::*;

use arrayvec::ArrayVec;

pub struct BSDF<'a> {
    pub p: Point3f,
    pub ns: Normal3f,
    pub ng: Normal3f,
    bxdfs: ArrayVec<[&'a mut dyn BxDF; 8]>,
}

impl<'a> BSDF<'a> {
    pub fn new(hit: &GeometryHitInfo) -> Self {
        Self {
            p: hit.point,
            ns: hit.ns,
            ng: hit.ng,
            bxdfs: ArrayVec::new()
        }
    }

    pub fn push<T: 'a + BxDF>(&mut self, alloc: &'a Bump, bxdf: T) {
        self.bxdfs.push(alloc.alloc(bxdf));
    }
}