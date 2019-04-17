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

    // Sampling method from http://jcgt.org/published/0007/04/01/paper.pdf#page=10
    // vec3 sampleGGXVNDF(vec3 Ve, float alpha_x, float alpha_y, float U1, float U2)
    fn sample(&self, wo: ShadingVec3f, samples: (Float, Float)) -> ShadingVec3f {
        let (ax, ay) = (self.alpha_x, self.alpha_y);

        // Section 3.2: transforming the view direction to the hemisphere configuration
        // vec3 Vh = normalize(vec3(alpha_x * Ve.x, alpha_y * Ve.y, Ve.z));
        let vh = ShadingVec3f::new(ax * wo.x, ay * wo.y, wo.z).normalized();

        // Section 4.1: orthonormal basis
        // vec3 T1 = (Vh.z < 0.9999) ? normalize(cross(vec3(0, 0, 1), Vh)) : vec3(1, 0, 0);
        let t1_v = if vh.z < 0.9999 {
            ShadingVec3f::new(0.0, 0.0, 0.1).cross(vh).normalized()
        } else {
            ShadingVec3f::new(1.0, 0.0, 0.0)
        };
        // vec3 T2 = cross(Vh, T1);
        let t2_v = wo.cross(t1_v);

        // Section 4.2: parameterization of the projected area
        // float r = sqrt(U1);
        let r = samples.0.sqrt();
        // float phi = 2.0 * M_PI * U2;
        let phi = 2.0 * Float::PI() * samples.1;
        // float t1 = r * cos(phi);
        let t1 = r * phi.cos();
        // float t2 = r * sin(phi);
        let t2 = r * phi.sin();
        // float s = 0.5 * (1.0 + Vh.z);
        let s = 0.5 * (1.0 + vh.z);
        // t2 = (1.0 - s)*sqrt(1.0 - t1*t1) + s*t2;
        let t2 = (1.0 - s) * (1.0 - t1 * t1).sqrt() + s * t2;

        // Section 4.3: reprojection onto hemisphere
        // vec3 Nh = t1*T1 + t2*T2 + sqrt(max(0.0, 1.0 - t1*t1 - t2*t2))*Vh;
        let nh = t1_v * t1 + t2_v * t2 + vh * (1.0 - t1 * t1 - t2 * t2).max(0.0).sqrt();

        // Section 3.4: transforming the normal back to the ellipsoid configuration
        // return normalize(vec3(alpha_x * Nh.x, alpha_y * Nh.y, std::max<float>(0.0, Nh.z)));
        ShadingVec3f::new(ax * nh.x, ay * nh.y, nh.z.max(0.0)).normalized()
    }

    fn pdf(&self, wo: ShadingVec3f, wh: ShadingVec3f) -> Float {
        self.distribution(wh) * self.g1(wo) * wo.dot(wh).abs() / wo.cos_theta().abs()
    }
}
