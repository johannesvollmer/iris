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

    fn eval(&self, _wi: &LocalVec3f, _wo: &LocalVec3f) -> Spectrum {
        Spectrum::all(0.0)
    }

    fn sample(&self, wo: &LocalVec3f, _samples: (f32, f32)) -> (Spectrum, LocalVec3f, Float) {
        let wi = LocalVec3f::new(-wo.x, -wo.y, wo.z);

        let spectrum = if wi.abs_cos_theta() == 0.0 {
            Spectrum::all(0.0)
        } else {
            self.fresnel.fresnel(wi.cos_theta()) * self.r / wi.abs_cos_theta()
        };

        (spectrum, wi, 1.0)
    }

    fn pdf(&self, _wi: &LocalVec3f, _wo: &LocalVec3f) -> Float {
        0.0
    }
}
