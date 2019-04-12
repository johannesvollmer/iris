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

    fn sample(&self, wo: ShadingVec3f, samples: (Float, Float)) -> ShadingVec3f {
        let flip = wo.z < 0.0;
        if flip {
            -ggx_sample(-wo, self.alpha_x, self.alpha_y, samples)
        } else {
            ggx_sample(wo, self.alpha_x, self.alpha_y, samples)
        }
    }

    fn pdf(&self, wo: ShadingVec3f, wh: ShadingVec3f) -> Float {
        self.distribution(wh) * self.g1(wo) * wo.dot(wh).abs() / wo.cos_theta().abs()
    }
}

fn ggx_sample_l1(cos_theta: Float, samples: (Float, Float)) -> (Float, Float) {
    if cos_theta > 0.9999 {
        let r = (samples.0 / (1.0 - samples.0)).sqrt();
        let phi = 2.0 * Float::PI() * samples.1;
        return (r * phi.cos(), r * phi.sin());
    }

    let sin_theta = (1.0 - cos_theta * cos_theta).max(0.0).sqrt();
    let tan_theta = sin_theta / cos_theta;
    let a = 1.0 / tan_theta;
    let g1 = 2.0 / (1.0 + (1.0 + 1.0 / (a * a)).sqrt());

    let a = 2.0 * samples.0 / g1 - 1.0;
    let b = tan_theta;
    let tmp = (1.0 / (a * a - 1.0)).min(1e10);
    let d = (b * b * tmp * tmp - (a * a - b * b) * tmp).max(0.0).sqrt();

    let slope_x = if a < 0.0 || (b * tmp + d) > 1.0 / tan_theta {
        b * tmp - d
    } else {
        b * tmp + d
    };

    let (s, new_u2) = if samples.1 > 0.5 {
        (1.0, 2.0 * samples.1 - 1.0)
    } else {
        (-1.0, 1.0 - 2.0 * samples.1)
    };

    let z = (new_u2 * (new_u2 * (new_u2 * 0.027385 - 0.073369) + 0.46341))
        / (new_u2 * (new_u2 * (new_u2 * 0.093073 + 0.309420) - 1.0) + 0.597999);
    let slope_y = s * z * (1.0 + slope_x * slope_x).sqrt();

    debug_assert!(slope_y.is_finite());
    (slope_x, slope_y)
}

fn ggx_sample(wi: ShadingVec3f, ax: Float, ay: Float, samples: (Float, Float)) -> ShadingVec3f {
    let wi_stretched = ShadingVec3f::new(ax * wi.x, ay * wi.y, wi.z).normalized();

    let (slope_x, slope_y) = ggx_sample_l1(wi_stretched.cos_theta(), samples);

    let tmp = wi_stretched.cos_phi() * slope_x - wi_stretched.sin_phi() * slope_y;
    let slope_y = wi_stretched.sin_phi() * slope_x + wi_stretched.cos_phi() * slope_y;
    let slope_x = tmp;

    ShadingVec3f::new(-slope_x * ax, -slope_y * ay, 1.0)
}
