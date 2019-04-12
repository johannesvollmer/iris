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
        #[allow(clippy::identity_op)]
        const REFLECTION = 1 << 0;
        const TRANSMISSION = 1 << 1;
        const DIFFUSE = 1 << 2;
        const GLOSSY = 1 << 3;
        const SPECULAR = 1 << 4;
        const ALL = Self::REFLECTION.bits | Self::TRANSMISSION.bits | Self::DIFFUSE.bits | Self::GLOSSY.bits | Self::SPECULAR.bits;
    }
}

impl BxDFType {
    pub fn for_hemisphere(mut self, wo: ShadingVec3f, wi: ShadingVec3f) -> Self {
        let flag_to_clear = if wi.same_hemisphere(wo) {
            BxDFType::TRANSMISSION
        } else {
            BxDFType::REFLECTION
        };

        self.set(flag_to_clear, false);
        self
    }
}

pub trait BxDF {
    fn get_type(&self) -> BxDFType;

    fn matches(&self, t: BxDFType) -> bool {
        t.contains(self.get_type())
    }

    fn eval(&self, wi: ShadingVec3f, wo: ShadingVec3f) -> Spectrum;

    fn sample(&self, wo: ShadingVec3f, samples: (Float, Float)) -> (Spectrum, ShadingVec3f, Float) {
        let mut wi = sample::cos_hemisphere(samples).as_shading();

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

    #[allow(dead_code)]
    fn rho_dir(&self, w: ShadingVec3f, samples: &[(Float, Float)]) -> Spectrum {
        samples
            .iter()
            .filter_map(|&sample| {
                let (f, wi, pdf) = self.sample(w, sample);
                if pdf > 0.0 {
                    Some(f * wi.cos_theta().abs() / pdf)
                } else {
                    None
                }
            })
            .sum::<Spectrum>()
            / samples.len() as Float
    }

    #[allow(dead_code)]
    fn rho_hemisphere(
        &self,
        samples_a: &[(Float, Float)],
        samples_b: &[(Float, Float)],
    ) -> Spectrum {
        samples_a
            .iter()
            .zip(samples_b.iter())
            .filter_map(|(&a, &b)| {
                let wo = sample::uniform_hemisphere(a).as_shading();
                let (f, wi, pdf) = self.sample(wo, b);
                let numerator = f * (wi.cos_theta() * wo.cos_theta()).abs();
                let denominator = pdf * sample::uniform_hemisphere_pdf();
                if pdf > 0.0 {
                    Some(numerator / denominator)
                } else {
                    None
                }
            })
            .sum::<Spectrum>()
            / (samples_a.len() as Float * Float::PI())
    }
}
