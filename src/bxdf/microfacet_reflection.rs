use crate::bxdf::fresnel::Fresnel;
use crate::bxdf::microfacet::MicrofacetDistribution;
use crate::bxdf::BxDF;
use crate::bxdf::BxDFType;
use crate::film::spectrum::Spectrum;
use crate::math::*;
use std::sync::Arc;

#[derive(new)]
pub struct MicrofacetReflection {
    r: Spectrum,
    distribution: Arc<dyn MicrofacetDistribution + Send + Sync>,
    fresnel: Arc<dyn Fresnel + Send + Sync>,
}

impl BxDF for MicrofacetReflection {
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

    fn sample(
        &self,
        _wo: ShadingVec3f,
        _samples: (Float, Float),
    ) -> (Spectrum, ShadingVec3f, Float) {
        unimplemented!()
    }

    fn pdf(&self, _wi: ShadingVec3f, _wo: ShadingVec3f) -> Float {
        unimplemented!()
    }
}
