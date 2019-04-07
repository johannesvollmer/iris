use crate::bxdf::BxDF;
use crate::bxdf::BxDFType;
use crate::film::spectrum::Spectrum;
use crate::math::*;
use num::traits::FloatConst;

pub struct OrenNayar {
    r: Spectrum,
    a: Float,
    b: Float,
}

impl OrenNayar {
    pub fn new(r: Spectrum, sigma_deg: Float) -> Self {
        let sigma_2 = sigma_deg.to_radians().powi(2);
        let a = 1.0 - (sigma_2 / (2.0 * (sigma_2 + 0.33)));
        let b = 0.45 * sigma_2 / (sigma_2 + 0.09);

        Self { r, a, b }
    }
}

impl BxDF for OrenNayar {
    fn get_type(&self) -> BxDFType {
        BxDFType::REFLECTION | BxDFType::DIFFUSE
    }

    fn eval(&self, wi: ShadingVec3f, wo: ShadingVec3f) -> Spectrum {
        let sin_theta_i = wi.sin_theta();
        let sin_theta_o = wo.sin_theta();

        let max_cos = if sin_theta_i > 1e-4 && sin_theta_o > 1e-4 {
            let (sin_phi_i, cos_phi_i) = (wi.sin_phi(), wi.cos_phi());
            let (sin_phi_o, cos_phi_o) = (wo.sin_phi(), wo.cos_phi());
            let dcos = cos_phi_i * cos_phi_o + sin_phi_i * sin_phi_o;
            dcos.max(0.0)
        } else {
            0.0
        };

        let (sin_alpha, tan_beta) = if wi.cos_theta().abs() > wo.cos_theta().abs() {
            (sin_theta_o, sin_theta_i / wi.cos_theta().abs())
        } else {
            (sin_theta_i, sin_theta_o / wo.cos_theta().abs())
        };

        self.r * Float::FRAC_1_PI() * (self.a + self.b * max_cos * sin_alpha * tan_beta)
    }
}
