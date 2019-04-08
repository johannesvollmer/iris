use crate::bxdf::fresnel::Fresnel;
use crate::bxdf::BxDF;
use crate::bxdf::BxDFType;
use crate::film::spectrum::Spectrum;
use crate::math::*;
use std::sync::Arc;

#[derive(new)]
pub struct SpecularReflection {
    pub r: Spectrum,
    pub fresnel: Arc<dyn Fresnel>,
}

impl BxDF for SpecularReflection {
    fn get_type(&self) -> BxDFType {
        BxDFType::REFLECTION | BxDFType::SPECULAR
    }

    fn eval(&self, _wi: ShadingVec3f, _wo: ShadingVec3f) -> Spectrum {
        Spectrum::all(0.0)
    }

    fn sample(
        &self,
        wo: ShadingVec3f,
        _samples: (Float, Float),
    ) -> (Spectrum, ShadingVec3f, Float) {
        let wi = ShadingVec3f::new(-wo.x, -wo.y, wo.z);

        let spectrum = if wi.cos_theta().abs() == 0.0 {
            Spectrum::all(0.0)
        } else {
            self.fresnel.fresnel(wi.cos_theta().abs()) * self.r / wi.cos_theta().abs()
        };

        (spectrum, wi, 1.0)
    }

    fn pdf(&self, _wi: ShadingVec3f, _wo: ShadingVec3f) -> Float {
        0.0
    }
}
