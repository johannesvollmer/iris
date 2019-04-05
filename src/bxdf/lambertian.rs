use crate::bxdf::BxDF;
use crate::bxdf::BxDFType;
use crate::film::spectrum::Spectrum;
use crate::math::*;
use num::traits::FloatConst;

#[derive(new)]
pub struct Lambertian {
    r: Spectrum,
}

impl BxDF for Lambertian {
    fn get_type(&self) -> BxDFType {
        BxDFType::REFLECTION | BxDFType::DIFFUSE
    }

    fn eval(&self, _wi: ShadingVec3f, _wo: ShadingVec3f) -> Spectrum {
        self.r * Float::FRAC_1_PI()
    }
}
