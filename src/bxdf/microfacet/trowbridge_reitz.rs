use super::MicrofacetDistribution;
use crate::math::*;
use num::traits::FloatConst;

#[derive(new, Copy, Clone)]
pub struct TrowbridgeReitz {
    alpha_x: Float,
    alpha_y: Float,
}

impl MicrofacetDistribution for TrowbridgeReitz {
    fn distribution(&self, wh: ShadingVec3f) -> Float {
        let tan_2_theta = wh.tan_2_theta();

        if tan_2_theta.is_infinite() {
            return 0.0;
        }

        let (ax, ay) = (self.alpha_x, self.alpha_y);
        let trig_inner = wh.cos_2_phi() / ax.powi(2) + wh.sin_2_phi() / ay.powi(2);
        let trig_outer = wh.cos_2_theta().powi(2) * (1.0 + tan_2_theta * trig_inner).powi(2);

        1.0 / (Float::PI() * ax * ay * trig_outer)
    }

    fn lambda(&self, w: ShadingVec3f) -> Float {
        let abs_tan_theta = w.tan_theta().abs();

        if abs_tan_theta.is_infinite() {
            return 0.0;
        }

        let (ax, ay) = (self.alpha_x, self.alpha_y);
        let alpha = (w.cos_2_phi() * ax.powi(2) + w.sin_2_phi() * ay.powi(2)).sqrt();
        let alpha_2_tan_2_theta = alpha.powi(2) * abs_tan_theta.powi(2);

        (-1.0 + (1.0 + alpha_2_tan_2_theta).sqrt()) / 2.0
    }
}
