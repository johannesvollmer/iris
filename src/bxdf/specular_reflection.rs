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

    fn eval(&self, _wi: &Vec3f, _wo: &Vec3f) -> Spectrum {
        Spectrum::all(0.0)
    }

    fn sample(&self, wo: &Vec3f, _samples: (f32, f32)) -> (Spectrum, Vec3f, Float) {
        let wi = Vec3f::new(-wo.x, -wo.y, wo.z);
        (
            self.fresnel.fresnel(wi.cos_theta()) * self.r / wi.abs_cos_theta(),
            wi,
            1.0,
        )
    }

    fn pdf(&self, _wi: &Vec3f, _wo: &Vec3f) -> Float {
        0.0
    }
}
