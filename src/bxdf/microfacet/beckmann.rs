use super::MicrofacetDistribution;
use crate::math::*;
use num::traits::FloatConst;

#[derive(new, Copy, Clone)]
pub struct Beckmann {
    alpha_x: Float,
    alpha_y: Float,
}

impl MicrofacetDistribution for Beckmann {
    fn distribution(&self, wh: ShadingVec3f) -> Float {
        let tan_2_theta = wh.tan_2_theta();

        if tan_2_theta.is_infinite() {
            return 0.0;
        }

        let (ax, ay) = (self.alpha_x, self.alpha_y);
        let denominator = Float::PI() * ax * ay * (wh.cos_2_theta().powi(2));
        let exponent = -tan_2_theta * (wh.cos_2_phi() / ax.powi(2) + wh.sin_2_phi() / ay.powi(2));

        exponent.exp() / denominator
    }

    fn lambda(&self, w: ShadingVec3f) -> Float {
        let abs_tan_theta = w.tan_theta().abs();

        if abs_tan_theta.is_infinite() {
            return 0.0;
        }

        let (ax, ay) = (self.alpha_x, self.alpha_y);
        let alpha = (w.cos_2_phi() * ax.powi(2) + w.sin_2_phi() * ay.powi(2)).sqrt();
        let a = 1.0 / (alpha * abs_tan_theta);

        if a >= 1.6 {
            0.0
        } else {
            (1.0 - 1.259 * a + 0.396 * a.powi(2)) / (3.535 * a + 2.181 * a.powi(2))
        }
    }
}
