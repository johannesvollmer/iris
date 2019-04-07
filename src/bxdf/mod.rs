use crate::film::spectrum::Spectrum;
use crate::math::*;
use num::traits::float::FloatConst;

pub mod bsdf;
pub mod fresnel;
pub mod lambertian;
pub mod microfacet;
pub mod microfacet_reflection;
pub mod oren_nayar;
pub mod specular_reflection;

bitflags! {
    pub struct BxDFType: u8 {
        const REFLECTION = 1 << 0;
        const TRANSMISSION = 1 << 1;
        const DIFFUSE = 1 << 2;
        const GLOSSY = 1 << 3;
        const SPECULAR = 1 << 4;
        const ALL = Self::REFLECTION.bits | Self::TRANSMISSION.bits | Self::DIFFUSE.bits | Self::GLOSSY.bits | Self::SPECULAR.bits;
    }
}

impl BxDFType {
    pub fn for_hemisphere(&self, wo: ShadingVec3f, wi: ShadingVec3f) -> Self {
        let flag_to_clear = if wi.same_hemisphere(wo) {
            BxDFType::TRANSMISSION
        } else {
            BxDFType::REFLECTION
        };

        let mut out = self.clone();
        out.set(flag_to_clear, false);
        out
    }
}

pub trait BxDF {
    fn get_type(&self) -> BxDFType;

    fn matches(&self, t: BxDFType) -> bool {
        t.contains(self.get_type())
    }

    fn eval(&self, wi: ShadingVec3f, wo: ShadingVec3f) -> Spectrum;

    fn sample(&self, wo: ShadingVec3f, samples: (Float, Float)) -> (Spectrum, ShadingVec3f, Float) {
        let mut wi = sample::cos_hemisphere(samples);

        if wo.z < 0.0 {
            wi.z *= -1.0;
        }

        (self.eval(wi, wo), wi, self.pdf(wi, wo))
    }

    fn pdf(&self, wi: ShadingVec3f, wo: ShadingVec3f) -> Float {
        if wo.same_hemisphere(wi) {
            wi.cos_theta().abs() * Float::FRAC_1_PI()
        } else {
            0.0
        }
    }
}
