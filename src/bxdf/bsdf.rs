use super::{BxDF, BxDFType};
use crate::film::spectrum::Spectrum;
use crate::geometry::SurfaceInteraction;
use crate::math::*;
use bumpalo::Bump;

use arrayvec::ArrayVec;

pub struct BSDF<'a> {
    pub p: Point3f,
    pub ns: Normal3f,
    pub ng: Normal3f,
    pub tan: Vec3f,
    pub bitan: Vec3f,
    bxdfs: ArrayVec<[&'a mut dyn BxDF; 8]>,
}

impl<'a> BSDF<'a> {
    pub fn new(hit: &SurfaceInteraction) -> Self {
        let bitan = hit.shading.dpdu.normalized();
        let tan = hit.shading.normal.cross(bitan.into());
        let bitan = tan.cross(hit.shading.normal);
        Self {
            p: hit.int.point,
            ns: hit.shading.normal,
            ng: hit.int.normal,
            bitan: bitan.into(),
            tan: tan.into(),
            bxdfs: ArrayVec::new(),
        }
    }

    pub fn push<T: 'a + BxDF>(&mut self, alloc: &'a Bump, bxdf: T) {
        self.bxdfs.push(alloc.alloc(bxdf));
    }

    fn vec_to_shading(&self, v: Vec3f) -> ShadingVec3f {
        ShadingVec3f::new(self.bitan.dot(v), self.tan.dot(v), self.ns.to_vec().dot(v))
    }

    fn vec_from_shading(&self, v: ShadingVec3f) -> Vec3f {
        Vec3f::new(
            self.bitan.x * v.x + self.tan.x * v.y + self.ns.x * v.z,
            self.bitan.y * v.x + self.tan.y * v.y + self.ns.y * v.z,
            self.bitan.z * v.x + self.tan.z * v.y + self.ns.z * v.z,
        )
    }

    fn num_matching(&self, types: BxDFType) -> usize {
        self.bxdfs.iter().filter(|bxdf| bxdf.matches(types)).count()
    }

    fn match_at(&self, types: BxDFType, i: usize) -> &&'a mut dyn BxDF {
        self.bxdfs
            .iter()
            .filter(|bxdf| bxdf.matches(types))
            .nth(i)
            .expect("BxDF out of bounds")
    }

    pub fn sample(
        &self,
        wo: Vec3f,
        types: BxDFType,
        samples: (Float, Float),
    ) -> (Spectrum, Vec3f, Float, BxDFType) {
        let empty_rv = (
            Spectrum::all(0.0),
            Vec3f::new(0.0, 0.0, 0.0),
            0.0,
            BxDFType::empty(),
        );

        let num_matching = self.num_matching(types);
        if num_matching == 0 {
            return empty_rv;
        }

        let component =
            ((samples.0.floor() * num_matching as Float) as usize).min(num_matching - 1);
        let bxdf = self.match_at(types, component);

        let wo_local = self.vec_to_shading(wo).normalized();
        let (mut spectrum, wi_local, mut pdf) = bxdf.sample(wo_local, samples);
        if wi_local.length_squared() == 0.0 {
            return empty_rv;
        }

        let wi = self.vec_from_shading(wi_local).normalized();
        if !bxdf.get_type().contains(BxDFType::SPECULAR) {
            if num_matching > 1 {
                // Compute total PDF
                pdf += self
                    .bxdfs
                    .iter()
                    .enumerate()
                    .filter(|(i, bxdf)| *i != component && bxdf.matches(types))
                    .map(|(_, bxdf)| bxdf.pdf(wo_local, wi_local))
                    .sum::<Float>();
            }

            // Remove appropriate flags if in different hemisphere
            let flags = types.for_hemisphere(wo_local, wi_local);

            // Compute total sample
            spectrum = self
                .bxdfs
                .iter()
                .filter(|bxdf| bxdf.matches(flags))
                .map(|bxdf| bxdf.eval(wo_local, wi_local))
                .sum()
        }

        if num_matching > 1 {
            pdf /= num_matching as Float;
        }

        (spectrum, wi, pdf, bxdf.get_type())
    }

    pub fn eval(&self, wo: Vec3f, wi: Vec3f, flags: BxDFType) -> Spectrum {
        let wo_local = self.vec_to_shading(wo).normalized();
        let wi_local = self.vec_to_shading(wi).normalized();

        let flags = flags.for_hemisphere(wo_local, wi_local);
        // dbg!(wi_local.cos_theta());

        self.bxdfs
            .iter()
            .filter(|bxdf| bxdf.matches(flags))
            .map(|bxdf| bxdf.eval(wo_local, wi_local))
            .sum()
    }

    pub fn pdf(&self, wo: Vec3f, wi: Vec3f, flags: BxDFType) -> Float {
        let wo_local = self.vec_to_shading(wo).normalized();
        let wi_local = self.vec_to_shading(wi).normalized();

        let (n_components, pdf) = self
            .bxdfs
            .iter()
            .filter(|bxdf| bxdf.matches(flags))
            .map(|bxdf| bxdf.pdf(wo_local, wi_local))
            .enumerate()
            .fold((0, 0.0), |(_, pdf_acc), (i, pdf)| (i, pdf_acc + pdf));

        if n_components > 0 {
            pdf / n_components as Float
        } else {
            0.0
        }
    }
}
