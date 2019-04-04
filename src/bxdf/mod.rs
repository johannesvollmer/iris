use crate::film::spectrum::Spectrum;
use crate::math::*;
use num::traits::float::FloatConst;

pub mod bsdf;
pub mod fresnel;
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

pub trait BxDF {
    fn get_type(&self) -> BxDFType;

    fn matches(&self, t: BxDFType) -> bool {
        self.get_type().contains(t)
    }

    fn eval(&self, wi: &LocalVec3f, wo: &LocalVec3f) -> Spectrum;

    fn sample(&self, wo: &LocalVec3f, samples: (f32, f32)) -> (Spectrum, LocalVec3f, Float) {
        let mut wi = sample::cos_hemisphere(samples);

        if wo.z < 0.0 {
            wi.z *= -1.0;
        }

        (self.eval(&wi, &wo), wi, self.pdf(&wi, &wo))
    }

    fn pdf(&self, wi: &LocalVec3f, wo: &LocalVec3f) -> Float {
        if wo.same_hemisphere(wi) {
            wi.abs_cos_theta() * Float::FRAC_1_PI()
        } else {
            0.0
        }
    }
}
