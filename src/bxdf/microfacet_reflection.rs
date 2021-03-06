use crate::bxdf::fresnel::Fresnel;
use crate::bxdf::microfacet::MicrofacetDistribution;
use crate::bxdf::BxDF;
use crate::bxdf::BxDFType;
use crate::film::spectrum::Spectrum;
use crate::math::*;

#[derive(new)]
pub struct MicrofacetReflection<'a> {
    r: Spectrum,
    distribution: &'a dyn MicrofacetDistribution,
    fresnel: &'a dyn Fresnel,
}

impl BxDF for MicrofacetReflection<'_> {
    fn get_type(&self) -> BxDFType {
        BxDFType::REFLECTION | BxDFType::GLOSSY
    }

    fn eval(&self, wi: ShadingVec3f, wo: ShadingVec3f) -> Spectrum {
        let cos_theta_o = wo.cos_theta().abs();
        let cos_theta_i = wi.cos_theta().abs();
        let wh = wi + wo;

        if cos_theta_i == 0.0 || cos_theta_o == 0.0 || wh == 0.0 {
            return Spectrum::all(0.0);
        }

        let wh = wh.normalized();
        let f = self.fresnel.fresnel(wi.dot(wh));

        self.r * self.distribution.distribution(wh) * self.distribution.g(wo, wi) * f
            / (4.0 * cos_theta_i * cos_theta_o)
    }

    fn sample(&self, wo: ShadingVec3f, samples: (Float, Float)) -> (Spectrum, ShadingVec3f, Float) {
        let wh = self.distribution.sample(wo, samples);
        let wi = ShadingVec3f::reflect(wo, wo);
        if !wo.same_hemisphere(wi) {
            return (Spectrum::default(), ShadingVec3f::default(), 0.0);
        }

        let pdf = self.distribution.pdf(wo, wh) / (4.0 * wo.dot(wh));
        (self.eval(wo, wi), wi, pdf)
    }

    fn pdf(&self, wi: ShadingVec3f, wo: ShadingVec3f) -> Float {
        if !wo.same_hemisphere(wi) {
            0.0
        } else {
            let wh = (wo + wi).normalized();
            self.distribution.pdf(wo, wh) / (4.0 * wo.dot(wh))
        }
    }
}
